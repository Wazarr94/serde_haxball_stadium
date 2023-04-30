use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GoalRaw {
    p0: [f32; 2],
    p1: [f32; 2],
    team: String,
}
