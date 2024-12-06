use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let input = std::env::args().nth(1).expect("missing input");

    let mut search_map: Matrix<char> = Matrix::new();

    if let Ok(lines) = read_lines(input) {
        let mut cols = -1;
        let mut rows = 0;
        for line in lines.flatten() {
            if cols == -1 {
                cols = line.len() as i32;
            }
            for char in line.chars() {
                search_map.data.push(char);
            }
            rows += 1;
        }
        search_map.width = cols;
        search_map.height = rows;
    }

    let start = search_map.find('^');
    if start.is_none() {
        return;
    }
    let guard = start.unwrap();
    // pointing up
    let dir = Vector { x: 0, y: -1 };

    let visited = move_guard(&search_map, guard, dir);
    let steps = visited.data.iter().filter(|&p| *p == 'X').count();

    println!("answer: {}", steps);

    let mut loops = 0;

    for c in search_map.iter() {
        let tile = search_map.at_coord(c).unwrap();

        if *tile == '#' {
            // already blocked
            continue;
        }

        let mut find_loop = search_map.clone();
        find_loop.set(c, 'O');
        if detect_loop(find_loop, guard, dir) {
            loops += 1;
        }
    }

    println!("answer: {}", loops);
}

fn move_guard(m: &Matrix<char>, mut guard: Coord, mut dir: Vector) -> Matrix<char> {
    let mut visited = m.clone();

    loop {
        if !m.in_coord(guard) {
            break;
        }

        let in_front = guard.add(dir);

        // new tile outside of search map
        if !m.in_coord(in_front) {
            visited.set(guard, 'X');
            guard.update(dir);
            continue;
        }

        let tile = m.at_coord(in_front);

        if *tile.unwrap() == '#' {
            dir = dir.rotate_90_cw();
            continue;
        }

        visited.set(guard, 'X');
        guard.update(dir);
    }

    return visited;
}

fn detect_loop(mut m: Matrix<char>, orig: Coord, mut dir: Vector) -> bool {
    let mut visited: HashMap<Coord, Vec<Vector>> = HashMap::new();
    let mut curr = orig.clone();

    loop {
        m.set(curr, 'X');

        let next = curr.add(dir);

        if !m.in_coord(next) {
            return false;
        }

        let tile = m.at_coord(next).unwrap();

        if *tile == '#' || *tile == 'O' {
            dir = dir.rotate_90_cw();

            let t = visited.get(&curr);
            if t.is_some() {
                if t.unwrap().contains(&dir) {
                    return true;
                }
            }

            visited
                .entry(curr)
                .and_modify(|v| v.push(dir))
                .or_insert(vec![dir]);
            continue;
        }

        curr = next;
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Clone)]
struct Matrix<T> {
    data: Vec<T>,
    width: i32,
    height: i32,
}

impl<T: std::clone::Clone + std::cmp::PartialEq> Matrix<T> {
    fn new() -> Matrix<T> {
        return Matrix {
            data: Vec::new(),
            width: 0,
            height: 0,
        };
    }

    fn find(&self, d: T) -> Option<Coord> {
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

    fn in_coord(&self, c: Coord) -> bool {
        return c.x >= 0 && c.x < self.width && c.y >= 0 && c.y < self.height;
    }

    fn at_coord(&self, c: Coord) -> Option<&T> {
        let pos = c.y * self.width + c.x;
        return self.data.get(pos as usize);
    }

    fn set(&mut self, c: Coord, v: T) {
        let pos = c.y * self.width + c.x;
        self.data[pos as usize] = v;
    }

    fn iter(&self) -> MatrixIterator<T> {
        return MatrixIterator {
            matrix: self,
            pos_iter: 0,
        };
    }
}

impl<T: fmt::Display> fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (pos, tile) in self.data.iter().enumerate() {
            if pos as i32 % self.width == 0 {
                writeln!(f)?;
            }
            write!(f, "{}", tile)?;
        }
        Ok(())
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

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
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

    fn update(&mut self, v: Vector) {
        self.x += v.x;
        self.y += v.y;
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Copy, Clone, PartialEq)]
struct Vector {
    x: i32,
    y: i32,
}

impl Vector {
    fn rotate_90_cw(&self) -> Vector {
        return Vector {
            x: -self.y,
            y: self.x,
        };
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
