use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let input = std::env::args().nth(1).expect("missing input");

    let mut word_search: Matrix<char> = Matrix::new();

    if let Ok(lines) = read_lines(input) {
        let mut cols = -1;
        let mut rows = 0;
        for line in lines.flatten() {
            if cols == -1 {
                cols = line.len() as i32;
            }
            for char in line.chars() {
                word_search.data.push(char);
            }
            rows += 1;
        }
        word_search.width = cols;
        word_search.height = rows;
    }

    let mut total = 0;

    for c in word_search.iter() {
        // println!("{}", c);

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

    let mut total = 0;

    for c in word_search.iter() {
        // println!("{}", c);

        let val = word_search.at_coord(c).unwrap();

        if *val != 'A' {
            continue;
        }

        if search_cross_mas(&word_search, c) {
            total += 1;
        }
    }

    println!("answer: {}", total);
}

fn search_xmas(m: &Matrix<char>, orig: Coord, dir: Vector) -> bool {
    let word = ['X', 'M', 'A', 'S'];

    for i in 0..word.len() {
        let curr = orig.add(dir.mul(i as i32));

        if !m.in_coord(curr) {
            return false;
        }

        let val = m.at_coord(curr);

        if *val.unwrap() != word[i] {
            return false;
        }
    }

    // println!("found at {} -> {}", orig, dir);

    return true;
}

fn search_cross_mas(m: &Matrix<char>, orig: Coord) -> bool {
    let directions = [UP_LEFT, UP_RIGHT, DOWN_LEFT, DOWN_RIGHT];

    let mut xs = HashMap::from([('M', 0), ('S', 0)]);
    for v in directions {
        let curr = orig.add(v);
        if !m.in_coord(curr) {
            return false;
        }

        let val = m.at_coord(curr);
        xs.entry(*val.unwrap()).and_modify(|v| *v += 1);
    }

    if *xs.get(&'M').unwrap() != 2 || *xs.get(&'S').unwrap() != 2 {
        return false;
    }

    let ul = m.at_coord(orig.add(UP_LEFT));
    let dr = m.at_coord(orig.add(DOWN_RIGHT));

    // check for MAM or SAS
    if ul.unwrap() == dr.unwrap() {
        return false;
    }

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
    width: i32,
    height: i32,
}

impl<T: std::clone::Clone> Matrix<T> {
    fn new() -> Matrix<T> {
        return Matrix {
            data: Vec::new(),
            width: 0,
            height: 0,
        };
    }

    fn at_coord(&self, c: Coord) -> Option<&T> {
        let pos = c.y * self.width + c.x;
        return self.data.get(pos as usize);
    }

    fn in_coord(&self, c: Coord) -> bool {
        return c.x >= 0 && c.x < self.width && c.y >= 0 && c.y < self.height;
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
        write!(f, "size: {}x{}", self.width, self.height)
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

        let c = Coord {
            x: self.pos_iter as i32 % self.matrix.width,
            y: self.pos_iter as i32 / self.matrix.width,
        };

        self.pos_iter += 1;

        return Some(c);
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

    fn add(&self, v: Vector) -> Coord {
        return Coord {
            x: self.x + v.x,
            y: self.y + v.y,
        };
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Copy, Clone)]
struct Vector {
    x: i32,
    y: i32,
}

impl Vector {
    const fn add(&self, v: Vector) -> Vector {
        return Vector {
            x: self.x + v.x,
            y: self.y + v.y,
        };
    }

    fn mul(&self, n: i32) -> Vector {
        return Vector {
            x: self.x * n,
            y: self.y * n,
        };
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

const LEFT: Vector = Vector { x: -1, y: 0 };
const RIGHT: Vector = Vector { x: 1, y: 0 };
const UP: Vector = Vector { x: 0, y: -1 };
const DOWN: Vector = Vector { x: 0, y: 1 };
const UP_LEFT: Vector = UP.add(LEFT);
const UP_RIGHT: Vector = UP.add(RIGHT);
const DOWN_LEFT: Vector = DOWN.add(LEFT);
const DOWN_RIGHT: Vector = DOWN.add(RIGHT);
