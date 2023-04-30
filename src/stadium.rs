use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::background::{Background, BackgroundRaw};
use crate::disc::{Disc, DiscRaw};
use crate::goal::{Goal, GoalRaw};
use crate::hx_trait::Trait;
use crate::plane::{Plane, PlaneRaw};
use crate::segment::SegmentRaw;
use crate::vertex::{Vertex, VertexRaw};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StadiumRaw {
    name: String,
    height: f32,
    width: f32,
    spawn_distance: f32,
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
        let traits: HashMap<String, Trait> = self.traits.clone();
        let bg: Background = self.bg.to_background();
        let discs: Vec<Disc> = self.discs.iter().map(|d| d.to_disc(&traits)).collect();
        let goals: Vec<Goal> = self.goals.iter().map(|g| g.to_goal()).collect();
        let vertexes: Vec<Vertex> = self.vertexes.iter().map(|v| v.to_vertex()).collect();
        let planes: Vec<Plane> = self.planes.iter().map(|p| p.to_plane()).collect();
        Stadium {
            name: self.name.clone(),
            traits,
            bg,
            discs,
            goals,
            vertexes,
            planes,
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
    pub traits: HashMap<String, Trait>,
}
