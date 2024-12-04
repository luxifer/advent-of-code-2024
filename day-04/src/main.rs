use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let input = std::env::args().nth(1).expect("missing input");

    let mut word_search: Matrix<char> = Matrix::new();

    if let Ok(lines) = read_lines(input) {
        let mut cols = 0;
        let mut rows = 0;
        for line in lines.flatten() {
            cols = line.len() as i32;
            rows += 1;
            for char in line.chars() {
                word_search.data.push(char);
            }
        }
        word_search.len.x = cols;
        word_search.len.y = rows;
    }

    println!("{}", word_search);

    let mut total = 0;

    for c in word_search.iter() {
        let val = word_search.at_coord(c).unwrap();

        if *val != 'X' {
            continue;
        }

        let directions = [
            UP, UP_RIGHT, RIGHT, DOWN_RIGHT, DOWN, DOWN_LEFT, LEFT, UP_LEFT,
        ];

        for d in directions {
            if search_xmas(&word_search, c, d) {
                total += 1;
            }
        }
    }

    println!("answer: {}", total);
}

fn search_xmas(m: &Matrix<char>, orig: Coord, dir: Dir) -> bool {
    let word = ['X', 'M', 'A', 'S'];

    for i in 0..word.len() {
        let curr = orig.add(dir.mul(i as i32));
        let val = m.at_coord(curr);
        if val.is_none() {
            return false;
        }

        if *val.unwrap() != word[i] {
            return false;
        }
    }

    // println!("found at {} -> {}", orig, dir);

    return true;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct Matrix<T> {
    data: Vec<T>,
    len: Coord,
}

impl<T: std::clone::Clone> Matrix<T> {
    fn new() -> Matrix<T> {
        return Matrix {
            data: Vec::new(),
            len: Coord::new(),
        };
    }

    fn at_coord(&self, c: Coord) -> Option<&T> {
        let pos = c.y * self.len.x + c.x;
        if pos < 0 || pos > self.len.x * self.len.y {
            return None;
        }
        return self.data.get(pos as usize);
    }

    fn iter(&self) -> MatrixIterator<T> {
        return MatrixIterator {
            matrix: self,
            pos_iter: 0,
        };
    }
}

impl<T> fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "size: {}x{}", self.len.x, self.len.y)
    }
}

struct MatrixIterator<'a, T> {
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

        let i = self.pos_iter as i32;
        self.pos_iter += 1;
        let x = i % self.matrix.len.x;
        let y = i / self.matrix.len.x;

        return Some(Coord { x, y });
    }
}

#[derive(Copy, Clone)]
pub struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    pub fn new() -> Coord {
        return Coord { x: 0, y: 0 };
    }

    fn add(&self, d: Dir) -> Coord {
        return Coord {
            x: self.x + d.x,
            y: self.y + d.y,
        };
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Copy, Clone)]
struct Dir {
    x: i32,
    y: i32,
}

impl Dir {
    const fn add(&self, d: Dir) -> Dir {
        return Dir {
            x: self.x + d.x,
            y: self.y + d.y,
        };
    }

    fn mul(&self, n: i32) -> Dir {
        return Dir {
            x: self.x * n,
            y: self.y * n,
        };
    }
}

impl fmt::Display for Dir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

const LEFT: Dir = Dir { x: -1, y: 0 };
const RIGHT: Dir = Dir { x: 1, y: 0 };
const UP: Dir = Dir { x: 0, y: 1 };
const DOWN: Dir = Dir { x: 0, y: -1 };
const UP_LEFT: Dir = UP.add(LEFT);
const UP_RIGHT: Dir = UP.add(RIGHT);
const DOWN_LEFT: Dir = DOWN.add(LEFT);
const DOWN_RIGHT: Dir = DOWN.add(RIGHT);
