use bevy::{math::Vec2, prelude::Color};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::{
    hx_trait::{Trait, Traitable},
    utils::{parse_collision, parse_color, CollisionFlag},
};

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

impl Traitable for DiscRaw {
    fn apply_trait(&self, traits: &HashMap<String, Trait>) -> DiscRaw {
        let tr_def: Trait = Trait::default();
        let tr_d: &Trait = match &self.hx_trait {
            Some(tr_name) => traits.get(tr_name).unwrap(),
            None => &tr_def,
        };
        let radius: Option<f32> = self.radius.or(tr_d.radius);
        let inv_mass: Option<f32> = self.inv_mass.or(tr_d.inv_mass);
        let damping: Option<f32> = self.damping.or(tr_d.damping);
        let b_coef: Option<f32> = self.b_coef.or(tr_d.b_coef);
        let color: Option<Value> = self.color.as_ref().or(tr_d.color.as_ref()).cloned();
        let c_group: Option<Vec<String>> = self.c_group.as_ref().or(tr_d.c_group.as_ref()).cloned();
        let c_mask: Option<Vec<String>> = self.c_mask.as_ref().or(tr_d.c_mask.as_ref()).cloned();
        let hx_trait: Option<String> = self.hx_trait.clone();
        DiscRaw {
            radius,
            inv_mass,
            damping,
            b_coef,
            color,
            c_group,
            c_mask,
            hx_trait,
            ..*self
        }
    }
}

impl DiscRaw {
    pub fn apply_default(&self) -> DiscRaw {
        // fill in missing values using the default impl
        let d_def: DiscRaw = DiscRaw::default();
        DiscRaw {
            pos: self.pos,
            speed: self.speed.or(d_def.speed),
            gravity: self.gravity.or(d_def.gravity),
            radius: self.radius.or(d_def.radius),
            inv_mass: self.inv_mass.or(d_def.inv_mass),
            damping: self.damping.or(d_def.damping),
            b_coef: self.b_coef.or(d_def.b_coef),
            color: self.color.as_ref().or(d_def.color.as_ref()).cloned(),
            c_group: self.c_group.as_ref().or(d_def.c_group.as_ref()).cloned(),
            c_mask: self.c_mask.as_ref().or(d_def.c_mask.as_ref()).cloned(),
            hx_trait: self.hx_trait.clone(),
        }
    }

    pub fn to_disc(&self, traits: &HashMap<String, Trait>) -> Disc {
        let disc_raw: DiscRaw = self.apply_trait(traits).apply_default();
        let position: Vec2 = Vec2::new(disc_raw.pos[0], disc_raw.pos[1]);
        let speed: Vec2 = Vec2::new(disc_raw.speed.unwrap()[0], disc_raw.speed.unwrap()[1]);
        let gravity: Vec2 = Vec2::new(disc_raw.gravity.unwrap()[0], disc_raw.gravity.unwrap()[1]);
        let radius: f32 = disc_raw.radius.unwrap();
        let inv_mass: f32 = disc_raw.inv_mass.unwrap();
        let damping: f32 = disc_raw.damping.unwrap();
        let b_coef: f32 = disc_raw.b_coef.unwrap();
        let color: Color = parse_color(&disc_raw.color.unwrap(), true);
        let c_group: CollisionFlag = parse_collision(&disc_raw.c_group.unwrap());
        let c_mask: CollisionFlag = parse_collision(&disc_raw.c_mask.unwrap());
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
    pub position: Vec2,
    pub speed: Vec2,
    pub gravity: Vec2,
    pub radius: f32,
    pub inv_mass: f32,
    pub damping: f32,
    pub b_coef: f32,
    pub color: Color,
    pub c_group: CollisionFlag,
    pub c_mask: CollisionFlag,
}
