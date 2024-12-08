use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let input = std::env::args().nth(1).expect("missing input");

    let mut antena_map: Matrix<char> = Matrix::new();

    if let Ok(lines) = read_lines(input) {
        let mut cols = -1;
        let mut rows = 0;
        for line in lines.flatten() {
            if cols == -1 {
                cols = line.len() as i32;
            }
            for char in line.chars() {
                antena_map.data.push(char);
            }
            rows += 1;
        }
        antena_map.width = cols;
        antena_map.height = rows;
    }

    let mut antenas: HashMap<char, Vec<Coord>> = HashMap::new();

    for c in antena_map.iter() {
        let tile = antena_map.at_coord(c).unwrap();

        if *tile == '.' {
            continue;
        }

        antenas
            .entry(*tile)
            .and_modify(|p| p.push(c))
            .or_insert(vec![c]);
    }

    let antinodes = search_antinodes(&antenas, &antena_map);
    let mut antinode_map = antena_map.clone();
    antinodes.iter().for_each(|p| antinode_map.set(*p, '#'));

    println!("{}", antinode_map);
    println!("answer: {}", antinodes.len());

    let antinodes = search_antinodes_harmonics(&antenas, &antena_map);
    let mut antinode_map = antena_map.clone();
    antinodes.iter().for_each(|p| antinode_map.set(*p, '#'));

    println!("{}", antinode_map);
    println!("answer: {}", antinodes.len());
}

fn search_antinodes_harmonics(
    antenas: &HashMap<char, Vec<Coord>>,
    antena_map: &Matrix<char>,
) -> Vec<Coord> {
    let mut antinodes: Vec<Coord> = Vec::new();

    for (_, coords) in antenas.iter() {
        for antena in coords.iter() {
            for other_antena in coords.iter() {
                let distance = dist(antena, other_antena);

                // same antena
                if distance.is_zero() {
                    continue;
                }

                let mut curr_antinode = antena.clone();

                loop {
                    let next_antinode = curr_antinode.add(distance);

                    if !antena_map.in_coord(next_antinode) {
                        break;
                    }

                    // antinode already there
                    if !antinodes.contains(&next_antinode) {
                        antinodes.push(next_antinode);
                    }

                    curr_antinode = next_antinode;
                }
            }
        }
    }

    return antinodes;
}

fn search_antinodes(antenas: &HashMap<char, Vec<Coord>>, antena_map: &Matrix<char>) -> Vec<Coord> {
    let mut antinodes: Vec<Coord> = Vec::new();

    for (_, coords) in antenas.iter() {
        for antena in coords.iter() {
            for other_antena in coords.iter() {
                let distance = dist(antena, other_antena);

                // same antena
                if distance.is_zero() {
                    continue;
                }

                let antinode = antena.add(distance.mul(2));

                if !antena_map.in_coord(antinode) {
                    continue;
                }

                // antinode already there
                if antinodes.contains(&antinode) {
                    continue;
                }

                antinodes.push(antinode);
            }
        }
    }

    return antinodes;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn dist(a: &Coord, b: &Coord) -> Vector {
    return Vector {
        x: b.x - a.x,
        y: b.y - a.y,
    };
}
#[derive(Clone)]
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

#[derive(Copy, Clone, Debug, PartialEq)]
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
    fn mul(&self, n: i32) -> Vector {
        return Vector {
            x: self.x * n,
            y: self.y * n,
        };
    }

    fn is_zero(&self) -> bool {
        return self.x == 0 && self.y == 0;
    }

    // fn inverse(&self) -> Vector {
    //     return Vector {
    //         x: -self.x,
    //         y: -self.y,
    //     };
    // }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
