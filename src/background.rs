use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BackgroundRaw {
    #[serde(rename = "type")]
    hx_type: Option<String>,
    width: Option<f32>,
    height: Option<f32>,
    kick_off_radius: f32,
    corner_radius: Option<f32>,
    color: Option<Value>,
}
