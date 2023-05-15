use crate::stadium::StadiumRaw;
use jsonc_parser::parse_to_serde_value;
use std::error::Error;
use std::fs;

pub mod background;
pub mod disc;
pub mod goal;
pub mod hx_trait;
pub mod plane;
pub mod segment;
pub mod stadium;
pub mod utils;
pub mod vertex;

fn main() -> Result<(), Box<dyn Error>> {
    for stadium_file in fs::read_dir("stadiums")? {
        let stadium_str = fs::read_to_string(stadium_file?.path())?;
        let stadium_value = parse_to_serde_value(&stadium_str, &Default::default())?.unwrap();
        let stadium_raw: StadiumRaw = serde_json::from_value(stadium_value)?;
        let stadium = stadium_raw.to_stadium();
        println!("Successfully read {}", &stadium.name);
    }
    Ok(())
}
