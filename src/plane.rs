use bevy::math::Vec2;
use serde::{Deserialize, Serialize};

use crate::utils::{parse_collision, CollisionFlag};
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

impl PlaneRaw {
    pub fn to_plane(&self) -> Plane {
        let plane_raw = PlaneRaw::default();
        let normal = Vec2::new(self.normal[0], self.normal[1]);
        let dist = self.dist;
        let b_coef = match self.b_coef {
            Some(b) => b,
            None => plane_raw.b_coef.unwrap(),
        };
        let c_group = match &self.c_group {
            Some(g) => parse_collision(g),
            None => parse_collision(&plane_raw.c_group.unwrap()),
        };
        let c_mask = match &self.c_mask {
            Some(m) => parse_collision(m),
            None => parse_collision(&plane_raw.c_mask.unwrap()),
        };
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
