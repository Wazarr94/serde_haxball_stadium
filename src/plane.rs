use bevy::math::Vec2;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    hx_trait::{Trait, Traitable},
    utils::{parse_collision, CollisionFlag},
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlaneRaw {
    normal: [f32; 2],
    dist: f32,
    b_coef: Option<f32>,
    c_group: Option<Vec<String>>,
    c_mask: Option<Vec<String>>,
    #[serde(rename = "trait")]
    hx_trait: Option<String>,
}

impl Default for PlaneRaw {
    fn default() -> Self {
        PlaneRaw {
            normal: [0.0, 0.0],
            dist: 0.0,
            b_coef: Some(1.0),
            c_group: Some(vec!["wall".to_string()]),
            c_mask: Some(vec!["all".to_string()]),
            hx_trait: None,
        }
    }
}

impl Traitable for PlaneRaw {
    fn apply_trait(&self, traits: &HashMap<String, Trait>) -> PlaneRaw {
        let tr_def: Trait = Trait::default();
        let tr_d: &Trait = match &self.hx_trait {
            Some(tr_name) => traits.get(tr_name).unwrap(),
            None => &tr_def,
        };
        let b_coef: Option<f32> = self.b_coef.or(tr_d.b_coef);
        let c_group: Option<Vec<String>> = self.c_group.as_ref().or(tr_d.c_group.as_ref()).cloned();
        let c_mask: Option<Vec<String>> = self.c_mask.as_ref().or(tr_d.c_mask.as_ref()).cloned();
        let hx_trait: Option<String> = self.hx_trait.clone();
        PlaneRaw {
            b_coef,
            c_group,
            c_mask,
            hx_trait,
            ..*self
        }
    }
}

impl PlaneRaw {
    pub fn apply_default(&self) -> PlaneRaw {
        let default = PlaneRaw::default();
        PlaneRaw {
            normal: self.normal,
            dist: self.dist,
            b_coef: self.b_coef.or(default.b_coef),
            c_group: self.c_group.as_ref().or(default.c_group.as_ref()).cloned(),
            c_mask: self.c_mask.as_ref().or(default.c_mask.as_ref()).cloned(),
            hx_trait: self.hx_trait.clone().or(default.hx_trait),
        }
    }

    pub fn to_plane(&self, traits: &HashMap<String, Trait>) -> Plane {
        let plane_raw: PlaneRaw = self.apply_trait(traits).apply_default();
        let normal: Vec2 = Vec2::new(plane_raw.normal[0], plane_raw.normal[1]);
        let dist: f32 = plane_raw.dist;
        let b_coef: f32 = plane_raw.b_coef.unwrap();
        let c_group: CollisionFlag = parse_collision(plane_raw.c_group.as_ref().unwrap());
        let c_mask: CollisionFlag = parse_collision(plane_raw.c_mask.as_ref().unwrap());
        Plane {
            normal,
            dist,
            b_coef,
            c_group,
            c_mask,
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Plane {
    pub normal: Vec2,
    pub dist: f32,
    pub b_coef: f32,
    pub c_group: CollisionFlag,
    pub c_mask: CollisionFlag,
}
