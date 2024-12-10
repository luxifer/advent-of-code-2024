use advent_of_code::matrix::coord;
use advent_of_code::matrix::matrix;
use advent_of_code::matrix::vector;
use advent_of_code::utils;
use std::collections::HashSet;

fn main() {
    let input = std::env::args().nth(1).expect("missing input");

    let mut topographic_map: matrix::Matrix<i32> = matrix::Matrix::new();

    if let Ok(lines) = utils::read_lines(input) {
        let mut cols = -1;
        let mut rows = 0;
        for line in lines.flatten() {
            if cols == -1 {
                cols = line.len() as i32;
            }
            for char in line.chars() {
                if char == '.' {
                    topographic_map.data.push(-1);
                } else {
                    topographic_map.data.push(char.to_digit(10).unwrap() as i32);
                }
            }
            rows += 1;
        }
        topographic_map.width = cols;
        topographic_map.height = rows;
    }

    let mut total = 0;

    for c in topographic_map.iter() {
        let height = topographic_map.at_coord(c).unwrap();

        if *height != 0 {
            continue;
        }

        let visited = count_destination(&topographic_map, c, 0).unwrap();
        total += visited.len();
    }

    println!("part 1: {}", total);

    let mut total = 0;

    for c in topographic_map.iter() {
        let height = topographic_map.at_coord(c).unwrap();

        if *height != 0 {
            continue;
        }

        total += count_distinct_trailheads(&topographic_map, c, 0);
    }

    println!("part 2: {}", total);
}

fn count_distinct_trailheads(
    topographic_map: &matrix::Matrix<i32>,
    pos: coord::Coord,
    target_height: i32,
) -> i32 {
    // out of map
    if !topographic_map.in_coord(pos) {
        return 0;
    }

    let height = topographic_map.at_coord(pos).unwrap();

    // no more steps
    if *height != target_height {
        return 0;
    }

    // println!(
    //     "{}{} -> {}",
    //     " ".repeat(target_height as usize),
    //     pos,
    //     target_height
    // );

    // end of trail
    if *height == 9 {
        return 1;
    }

    let cross: Vec<vector::Vector> = vec![UP, RIGHT, DOWN, LEFT];
    let mut rating = 0;

    for dir in cross.iter() {
        let next_pos = pos.add(*dir);
        rating += count_distinct_trailheads(&topographic_map, next_pos, target_height + 1);
    }

    return rating;
}

fn count_destination(
    topographic_map: &matrix::Matrix<i32>,
    pos: coord::Coord,
    target_height: i32,
) -> Option<HashSet<coord::Coord>> {
    // out of map
    if !topographic_map.in_coord(pos) {
        return None;
    }

    let height = topographic_map.at_coord(pos).unwrap();

    // no more steps
    if *height != target_height {
        return None;
    }

    // println!(
    //     "{}{} -> {}",
    //     " ".repeat(target_height as usize),
    //     pos,
    //     target_height
    // );

    // end of trail
    if *height == 9 {
        return Some(HashSet::from([pos]));
    }

    let cross: Vec<vector::Vector> = vec![UP, RIGHT, DOWN, LEFT];
    let mut visited: HashSet<coord::Coord> = HashSet::new();

    for dir in cross.iter() {
        let next_pos = pos.add(*dir);
        let next_visited = count_destination(&topographic_map, next_pos, target_height + 1);

        if next_visited.is_some() {
            next_visited
                .unwrap()
                .iter()
                .for_each(|p| _ = visited.insert(*p));
        }
    }

    return Some(visited);
}

const LEFT: vector::Vector = vector::Vector { x: -1, y: 0 };
const RIGHT: vector::Vector = vector::Vector { x: 1, y: 0 };
const UP: vector::Vector = vector::Vector { x: 0, y: -1 };
const DOWN: vector::Vector = vector::Vector { x: 0, y: 1 };
