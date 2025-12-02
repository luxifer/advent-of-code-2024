use super::coord::Coord;
use std::fmt;

#[derive(Clone)]
pub struct Matrix<T> {
    pub data: Vec<T>,
    pub width: i32,
    pub height: i32,
}

impl<T: std::clone::Clone + std::cmp::PartialEq + std::marker::Copy> Matrix<T> {
    pub fn new() -> Matrix<T> {
        return Matrix {
            data: Vec::new(),
            width: 0,
            height: 0,
        };
    }

    pub fn from_size(width: i32, height: i32, val: T) -> Self {
        return Matrix {
            width,
            height,
            data: vec![val].repeat((width * height) as usize),
        };
    }

    pub fn reset(&mut self, val: T) {
        self.data = vec![val].repeat((self.width * self.height) as usize)
    }

    pub fn at_coord(&self, c: Coord) -> Option<&T> {
        let pos = c.y * self.width + c.x;
        return self.data.get(pos as usize);
    }

    pub fn in_coord(&self, c: Coord) -> bool {
        return c.x >= 0 && c.x < self.width && c.y >= 0 && c.y < self.height;
    }

    pub fn iter(&'_ self) -> MatrixIterator<'_, T> {
        return MatrixIterator {
            matrix: self,
            pos_iter: 0,
        };
    }

    pub fn find(&self, d: T) -> Option<Coord> {
        let idx = self.data.iter().position(|p| *p == d);
        if idx.is_none() {
            return None;
        }

        let c = Coord {
            x: idx.unwrap() as i32 % self.width,
            y: idx.unwrap() as i32 / self.width,
        };

        return Some(c);
    }

    pub fn set(&mut self, c: Coord, v: T) {
        let pos = c.y * self.width + c.x;
        self.data[pos as usize] = v;
    }
}

impl<T: std::fmt::Display> fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, v) in self.data.iter().enumerate() {
            if i as i32 % self.width == 0 {
                writeln!(f)?
            }
            write!(f, "{}", v)?
        }
        Ok(())
    }
}

pub struct MatrixIterator<'a, T> {
    matrix: &'a Matrix<T>,
    pos_iter: usize,
}

impl<'a, T> Iterator for MatrixIterator<'a, T> {
    type Item = Coord;
    fn next(&mut self) -> Option<Self::Item> {
        let e = self.matrix.data.get(self.pos_iter);

        if e.is_none() {
            return None;
        }

        let c = Coord {
            x: self.pos_iter as i32 % self.matrix.width,
            y: self.pos_iter as i32 / self.matrix.width,
        };

        self.pos_iter += 1;

        return Some(c);
    }
}
