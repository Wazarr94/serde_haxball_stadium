use bitflags::bitflags;
use std::str;

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct CollisionFlag: u16 {
        const BALL = 1;
        const RED = 2;
        const BLUE = 4;
        const REDKO = 8;
        const BLUEKO = 16;
        const WALL = 32;
        const ALL = 63;
        const KICK = 64;
        const SCORE = 128;
        const C0 = 256;
        const C1 = 512;
        const C2 = 1024;
        const C3 = 2048;
  }
}

impl std::str::FromStr for CollisionFlag {
    type Err = bitflags::parser::ParseError;

    fn from_str(flags: &str) -> Result<Self, Self::Err> {
        // deal with none
        if flags == "none" {
            return Ok(Self::empty());
        }

        let upper_flags = flags.to_uppercase();
        Ok(Self(upper_flags.parse()?))
    }
}
