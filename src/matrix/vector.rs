use std::fmt;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Vector {
    pub x: i32,
    pub y: i32,
}

impl Vector {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub const fn add(&self, v: Vector) -> Vector {
        return Vector {
            x: self.x + v.x,
            y: self.y + v.y,
        };
    }

    pub fn mul(&self, n: i32) -> Vector {
        return Vector {
            x: self.x * n,
            y: self.y * n,
        };
    }

    pub fn rotate_90_cw(&self) -> Vector {
        return Vector {
            x: -self.y,
            y: self.x,
        };
    }

    pub fn rotate_90_aw(&self) -> Vector {
        return Vector {
            x: self.y,
            y: -self.x,
        };
    }

    pub fn is_zero(&self) -> bool {
        return self.x == 0 && self.y == 0;
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
