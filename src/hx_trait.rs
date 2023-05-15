use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// in the game files, the trait can have any properties
// in this implementation, we only care about optional properties from other structs
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Trait {
    pub vis: Option<bool>,
    pub b_coef: Option<f64>,
    pub radius: Option<f64>,
    pub inv_mass: Option<f64>,
    pub speed: Option<[f64; 2]>,
    pub gravity: Option<[f64; 2]>,
    pub damping: Option<f64>,
    pub c_group: Option<Vec<String>>,
    pub c_mask: Option<Vec<String>>,
    pub acceleration: Option<f64>,
    pub color: Option<Value>,
    pub bias: Option<f64>,
    pub curve: Option<f64>,
    pub curve_f: Option<f64>,
}

pub trait Traitable {
    fn apply_trait(&self, traits: &HashMap<String, Trait>) -> Self;
}
