use std::fs::{DirEntry, ReadDir};

use crate::{
    disc::Disc,
    stadium::{Stadium, StadiumRaw},
};

pub mod background;
pub mod disc;
pub mod goal;
pub mod hx_trait;
pub mod plane;
pub mod segment;
pub mod stadium;
pub mod utils;
pub mod vertex;

fn main() {
    let stadium_dir: ReadDir = std::fs::read_dir("stadiums").unwrap();
    for stadium_file in stadium_dir {
        let stadium_file: DirEntry = stadium_file.unwrap();
        let stadium_str: String = std::fs::read_to_string(stadium_file.path()).unwrap();
        let stadium_raw: StadiumRaw = serde_json::from_str(&stadium_str).unwrap();
        let stadium: Stadium = stadium_raw.to_stadium();
        let discs: Vec<Disc> = stadium.discs;
        println!("Successfully read {}", &stadium.name);
        println!("{:#?}", discs);
    }
}
