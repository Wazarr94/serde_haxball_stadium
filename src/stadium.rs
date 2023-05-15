use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::background::{Background, BackgroundRaw};
use crate::disc::{Disc, DiscRaw};
use crate::goal::{Goal, GoalRaw};
use crate::hx_trait::Trait;
use crate::plane::{Plane, PlaneRaw};
use crate::segment::{Segment, SegmentRaw};
use crate::vertex::{Vertex, VertexRaw};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StadiumRaw {
    name: String,
    height: f64,
    width: f64,
    spawn_distance: f64,
    bg: BackgroundRaw,
    discs: Vec<DiscRaw>,
    vertexes: Vec<VertexRaw>,
    segments: Vec<SegmentRaw>,
    goals: Vec<GoalRaw>,
    planes: Vec<PlaneRaw>,
    traits: HashMap<String, Trait>,
}

impl StadiumRaw {
    pub fn to_stadium(&self) -> Stadium {
        let traits = self.traits.clone();
        let bg = self.bg.to_background();
        let discs = self.discs.iter().map(|d| d.to_disc(&traits)).collect();
        let goals = self.goals.iter().map(|g| g.to_goal()).collect();
        let vertexes = self.vertexes.iter().map(|v| v.to_vertex(&traits)).collect();
        let planes = self.planes.iter().map(|p| p.to_plane(&traits)).collect();
        let segments = self
            .segments
            .iter()
            .map(|s| s.to_segment(&traits))
            .collect();
        Stadium {
            name: self.name.clone(),
            bg,
            discs,
            goals,
            vertexes,
            planes,
            segments,
        }
    }
}

pub struct Stadium {
    pub name: String,
    pub bg: Background,
    pub discs: Vec<Disc>,
    pub goals: Vec<Goal>,
    pub vertexes: Vec<Vertex>,
    pub planes: Vec<Plane>,
    pub segments: Vec<Segment>,
}
