use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TraitRaw {
    vis: Option<bool>,
    b_coef: Option<f32>,
    c_group: Option<Vec<String>>,
    c_mask: Option<Vec<String>>,
    radius: Option<f32>,
    inv_mass: Option<f32>,
}
