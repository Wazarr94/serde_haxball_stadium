use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SegmentRaw {
    v0: i32,
    v1: i32,
    b_coef: Option<f32>,
    curve: Option<f32>,
    curve_f: Option<f32>,
    bias: Option<f32>,
    c_group: Option<Vec<String>>,
    c_mask: Option<Vec<String>>,
    vis: Option<bool>,
    color: Option<Value>,
    #[serde(rename = "trait")]
    hx_trait: Option<String>,
}
