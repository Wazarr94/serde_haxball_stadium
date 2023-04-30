use bevy::math::Vec2;
use serde::{Deserialize, Serialize};

use crate::utils::{parse_collision, CollisionFlag};
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VertexRaw {
    x: f32,
    y: f32,
    b_coef: Option<f32>,
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

impl VertexRaw {
    pub fn to_vertex(&self) -> Vertex {
        let vertex_raw = VertexRaw::default();
        let position = Vec2::new(self.x, self.y);
        let b_coef = match self.b_coef {
            Some(b) => b,
            None => vertex_raw.b_coef.unwrap(),
        };
        let c_group = match &self.c_group {
            Some(g) => parse_collision(g),
            None => parse_collision(&vertex_raw.c_group.unwrap()),
        };
        let c_mask = match &self.c_mask {
            Some(m) => parse_collision(m),
            None => parse_collision(&vertex_raw.c_mask.unwrap()),
        };
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
    position: Vec2,
    b_coef: f32,
    c_group: CollisionFlag,
    c_mask: CollisionFlag,
}
