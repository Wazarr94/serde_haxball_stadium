use bevy::{math::Vec2, prelude::Color};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::utils::{parse_color, CollisionFlag};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiscRaw {
    pos: [f32; 2],
    speed: Option<[f32; 2]>,
    gravity: Option<[f32; 2]>,
    radius: Option<f32>,
    inv_mass: Option<f32>,
    damping: Option<f32>,
    b_coef: Option<f32>,
    color: Option<Value>,
    c_group: Option<Vec<String>>,
    c_mask: Option<Vec<String>>,
    #[serde(rename = "trait")]
    hx_trait: Option<String>,
}

impl Default for DiscRaw {
    fn default() -> Self {
        DiscRaw {
            pos: [0.0, 0.0],
            speed: Some([0.0, 0.0]),
            gravity: Some([0.0, 0.0]),
            radius: Some(8.0),
            inv_mass: Some(0.0),
            damping: Some(0.99),
            b_coef: Some(0.5),
            color: Some(Value::String("FFFFFF".to_string())),
            c_group: Some(vec!["all".to_string()]),
            c_mask: Some(vec!["all".to_string()]),
            hx_trait: None,
        }
    }
}

impl DiscRaw {
    pub fn to_disc(&self) -> Disc {
        // fill in missing values using the default impl
        let disc_raw = DiscRaw::default();
        let position = Vec2::new(self.pos[0], self.pos[1]);
        let speed = match self.speed {
            Some(s) => Vec2::new(s[0], s[1]),
            None => Vec2::new(disc_raw.speed.unwrap()[0], disc_raw.speed.unwrap()[1]),
        };
        let gravity = match self.gravity {
            Some(g) => Vec2::new(g[0], g[1]),
            None => Vec2::new(disc_raw.gravity.unwrap()[0], disc_raw.gravity.unwrap()[1]),
        };
        let radius = match self.radius {
            Some(r) => r,
            None => disc_raw.radius.unwrap(),
        };
        let inv_mass = match self.inv_mass {
            Some(m) => m,
            None => disc_raw.inv_mass.unwrap(),
        };
        let damping = match self.damping {
            Some(d) => d,
            None => disc_raw.damping.unwrap(),
        };
        let b_coef = match self.b_coef {
            Some(b) => b,
            None => disc_raw.b_coef.unwrap(),
        };
        let color: Color = match &self.color {
            Some(c) => parse_color(c, true),
            None => parse_color(&disc_raw.color.unwrap(), true),
        };
        let c_group = match &self.c_group {
            Some(cg) => cg
                .iter()
                .map(|s| s.parse().unwrap())
                .fold(CollisionFlag::empty(), |acc, x| acc | x),
            None => disc_raw
                .c_group
                .unwrap()
                .iter()
                .map(|s| s.parse().unwrap())
                .fold(CollisionFlag::empty(), |acc, x| acc | x),
        };
        let c_mask = match &self.c_mask {
            Some(cm) => cm
                .iter()
                .map(|s| s.parse().unwrap())
                .fold(CollisionFlag::empty(), |acc, x| acc | x),
            None => disc_raw
                .c_mask
                .unwrap()
                .iter()
                .map(|s| s.parse().unwrap())
                .fold(CollisionFlag::empty(), |acc, x| acc | x),
        };
        Disc {
            position,
            speed,
            gravity,
            radius,
            inv_mass,
            damping,
            b_coef,
            color,
            c_group,
            c_mask,
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Disc {
    position: Vec2,
    speed: Vec2,
    gravity: Vec2,
    radius: f32,
    inv_mass: f32,
    damping: f32,
    b_coef: f32,
    color: Color,
    c_group: CollisionFlag,
    c_mask: CollisionFlag,
}
