use super::vector::Vector;
use std::fmt;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub fn new() -> Coord {
        return Coord { x: 0, y: 0 };
    }

    pub fn add(&self, v: Vector) -> Coord {
        return Coord {
            x: self.x + v.x,
            y: self.y + v.y,
        };
    }

    pub fn update(&mut self, v: Vector) {
        self.x += v.x;
        self.y += v.y;
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
