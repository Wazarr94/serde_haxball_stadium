use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlaneRaw {
    normal: [f32; 2],
    dist: f32,
    #[serde(rename = "trait")]
    hx_trait: Option<String>,
    c_group: Option<Vec<String>>,
    c_mask: Option<Vec<String>>,
}
