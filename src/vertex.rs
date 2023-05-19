use bevy::math::DVec2;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    hx_trait::{Trait, Traitable},
    utils::{parse_collision, CollisionFlag},
};
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VertexRaw {
    x: f64,
    y: f64,
    b_coef: Option<f64>,
    c_group: Option<Vec<String>>,
    c_mask: Option<Vec<String>>,
    #[serde(rename = "trait")]
    hx_trait: Option<String>,
}

impl Default for VertexRaw {
    fn default() -> Self {
        VertexRaw {
            x: 0.0,
            y: 0.0,
            b_coef: Some(1.0),
            c_group: Some(vec!["wall".to_string()]),
            c_mask: Some(vec!["all".to_string()]),
            hx_trait: None,
        }
    }
}

impl Traitable for VertexRaw {
    fn apply_trait(&self, traits: &HashMap<String, Trait>) -> VertexRaw {
        let tr_def = Trait::default();
        let tr_d = match &self.hx_trait {
            Some(tr_name) => traits.get(tr_name).unwrap_or(&tr_def),
            None => &tr_def,
        };
        let b_coef = self.b_coef.or(tr_d.b_coef);
        let c_group = self.c_group.as_ref().or(tr_d.c_group.as_ref()).cloned();
        let c_mask = self.c_mask.as_ref().or(tr_d.c_mask.as_ref()).cloned();
        let hx_trait = self.hx_trait.clone();
        VertexRaw {
            b_coef,
            c_group,
            c_mask,
            hx_trait,
            ..*self
        }
    }
}

impl VertexRaw {
    pub fn apply_default(&self) -> VertexRaw {
        let default: VertexRaw = VertexRaw::default();
        VertexRaw {
            x: self.x,
            y: self.y,
            b_coef: self.b_coef.or(default.b_coef),
            c_group: self.c_group.as_ref().or(default.c_group.as_ref()).cloned(),
            c_mask: self.c_mask.as_ref().or(default.c_mask.as_ref()).cloned(),
            hx_trait: self.hx_trait.clone(),
        }
    }

    pub fn to_vertex(&self, traits: &HashMap<String, Trait>) -> Vertex {
        let vertex_raw = self.apply_trait(traits).apply_default();
        let position = DVec2::new(vertex_raw.x, vertex_raw.y);
        let b_coef = vertex_raw.b_coef.unwrap();
        let c_group = parse_collision(&vertex_raw.c_group.unwrap());
        let c_mask = parse_collision(&vertex_raw.c_mask.unwrap());
        Vertex {
            position,
            b_coef,
            c_group,
            c_mask,
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Vertex {
    pub position: DVec2,
    pub b_coef: f64,
    pub c_group: CollisionFlag,
    pub c_mask: CollisionFlag,
}
