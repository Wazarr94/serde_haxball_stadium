use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SegmentRaw {
    v0: i32,
    v1: i32,
    #[serde(rename = "trait")]
    hx_trait: Option<String>,
    curve: Option<f32>,
    c_group: Option<Vec<String>>,
    c_mask: Option<Vec<String>>,
}
