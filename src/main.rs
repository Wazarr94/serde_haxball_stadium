use std::collections::HashMap;
use std::fs::{DirEntry, ReadDir};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Vertex {
    x: f32,
    y: f32,
    #[serde(rename = "trait")]
    hx_trait: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Segment {
    v0: i32,
    v1: i32,
    #[serde(rename = "trait")]
    hx_trait: Option<String>,
    curve: Option<f32>,
    c_group: Option<Vec<String>>,
    c_mask: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Goal {
    p0: [f32; 2],
    p1: [f32; 2],
    team: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Disc {
    pos: [f32; 2],
    #[serde(rename = "trait")]
    hx_trait: Option<String>,
    color: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Plane {
    normal: [f32; 2],
    dist: f32,
    #[serde(rename = "trait")]
    hx_trait: Option<String>,
    c_group: Option<Vec<String>>,
    c_mask: Option<Vec<String>>,
}

// type is "none", "grass", "hockey" or not present
// we create the BackgroundType enum to represent this
// enum BackgroundType {
//     None,
//     Grass,
//     Hockey,
// }

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Background {
    #[serde(rename = "type")]
    hx_type: Option<String>,
    width: Option<f32>,
    height: Option<f32>,
    kick_off_radius: f32,
    corner_radius: Option<f32>,
    color: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Trait {
    vis: Option<bool>,
    b_coef: Option<f32>,
    c_group: Option<Vec<String>>,
    c_mask: Option<Vec<String>>,
    radius: Option<f32>,
    inv_mass: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Stadium {
    name: String,
    height: f32,
    width: f32,
    spawn_distance: f32,
    bg: Background,
    discs: Vec<Disc>,
    vertexes: Vec<Vertex>,
    segments: Vec<Segment>,
    goals: Vec<Goal>,
    planes: Vec<Plane>,
    traits: HashMap<String, Trait>,
}

fn main() {
    let stadium_dir: ReadDir = std::fs::read_dir("stadiums").unwrap();
    for stadium_file in stadium_dir {
        let stadium_file: DirEntry = stadium_file.unwrap();
        let stadium_str: String = std::fs::read_to_string(stadium_file.path()).unwrap();
        let stadium_json: Stadium = serde_json::from_str(&stadium_str).unwrap();
        println!("Successfully read {}", &stadium_json.name)
    }
}
