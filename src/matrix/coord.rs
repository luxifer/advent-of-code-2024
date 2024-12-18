use super::vector::Vector;
use std::fmt;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
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

    pub fn distance(&self, other: Coord) -> Vector {
        return Vector {
            x: other.x - self.x,
            y: other.y - self.y,
        };
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
