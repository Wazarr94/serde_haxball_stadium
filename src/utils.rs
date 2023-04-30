use bitflags::bitflags;

bitflags! {
    pub struct CollisionFlag: u32 {
        const NONE = 0x0;
        const BALL = 0x1;
        const RED = 0x2;
        const BLUE = 0x4;
        const REDKO = 0x8;
        const BLUEKO = 0x16;
        const WALL = 0x32;
        const ALL = 0x63;
        const KICK = 0x64;
        const SCORE = 0x128;
        const C0 = 0x256;
        const C1 = 0x512;
        const C2 = 0x1024;
        const C3 = 0x2048;
  }
}
