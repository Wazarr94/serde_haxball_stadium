use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// check in the game_min.js file if this is complete
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Trait {
    pub vis: Option<bool>,
    pub b_coef: Option<f32>,
    pub radius: Option<f32>,
    pub inv_mass: Option<f32>,
    pub gravity: Option<[f32; 2]>,
    pub damping: Option<f32>,
    pub c_group: Option<Vec<String>>,
    pub c_mask: Option<Vec<String>>,
    pub acceleration: Option<f32>,
    pub color: Option<Value>,
}

pub trait Traitable {
    fn apply_trait(&self, traits: &HashMap<String, Trait>) -> Self;
}
