use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::background::BackgroundRaw;
use crate::disc::{Disc, DiscRaw};
use crate::goal::GoalRaw;
use crate::hx_trait::TraitRaw;
use crate::plane::PlaneRaw;
use crate::segment::SegmentRaw;
use crate::vertex::VertexRaw;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StadiumRaw {
    pub name: String,
    height: f32,
    width: f32,
    spawn_distance: f32,
    bg: BackgroundRaw,
    discs: Vec<DiscRaw>,
    vertexes: Vec<VertexRaw>,
    segments: Vec<SegmentRaw>,
    goals: Vec<GoalRaw>,
    planes: Vec<PlaneRaw>,
    traits: HashMap<String, TraitRaw>,
}

impl StadiumRaw {
    pub fn to_stadium(&self) -> Stadium {
        let discs: Vec<Disc> = self.discs.iter().map(|d| d.to_disc()).collect();
        Stadium {
            name: self.name.clone(),
            discs,
        }
    }
}

pub struct Stadium {
    pub name: String,
    pub discs: Vec<Disc>,
}
