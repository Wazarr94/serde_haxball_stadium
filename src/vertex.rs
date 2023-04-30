use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct VertexRaw {
    x: f32,
    y: f32,
    #[serde(rename = "trait")]
    hx_trait: Option<String>,
}
