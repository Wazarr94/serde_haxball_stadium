use crate::utils::Team;
use bevy::math::Vec2;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GoalRaw {
    p0: [f32; 2],
    p1: [f32; 2],
    team: String,
}

impl GoalRaw {
    pub fn to_goal(&self) -> Goal {
        Goal {
            p0: Vec2::new(self.p0[0], self.p0[1]),
            p1: Vec2::new(self.p1[0], self.p1[1]),
            team: match self.team.as_str() {
                "red" => Team::Red,
                "blue" => Team::Blue,
                _ => panic!("Invalid team name"),
            },
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Goal {
    pub p0: Vec2,
    pub p1: Vec2,
    pub team: Team,
}
