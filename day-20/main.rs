use std::collections::HashSet;

use advent_of_code::cli;
use advent_of_code::matrix::coord;
use advent_of_code::matrix::matrix;
use advent_of_code::matrix::vector;
use anyhow::Result;
use pathfinding::prelude::astar;

fn main() -> Result<()> {
    let app = cli::Cli::new();
    let lines = app.content()?;

    let mut racetrack: matrix::Matrix<char> = matrix::Matrix::new();

    let mut cols = -1;
    let mut rows = 0;
    for line in lines.lines() {
        if cols == -1 {
            cols = line.len() as i32;
        }
        for char in line.chars() {
            racetrack.data.push(char);
        }
        rows += 1;
    }
    racetrack.width = cols;
    racetrack.height = rows;

    cli::stage(1, || -> i64 {
        let start = racetrack.find('S').unwrap();
        let goal = racetrack.find('E').unwrap();

        let (path, reference_time) = find_reference(&racetrack, start, goal).unwrap();

        println!("reference: {}", reference_time);

        let cheat_candidates = find_candidates(&racetrack, &path);

        println!("candidates: {}", cheat_candidates.len());

        let mut total = 0;

        for cheat_start_pos in cheat_candidates.iter() {
            let res = find_path(&racetrack, start, goal, &path, *cheat_start_pos, 2);

            match res {
                Some((_, picoseconds)) => {
                    let saved_time = reference_time - picoseconds;
                    if saved_time >= 100 {
                        total += 1;
                    }
                }
                None => (),
            }
        }

        return total;
    });

    cli::stage(2, || -> i64 {
        let start = racetrack.find('S').unwrap();
        let goal = racetrack.find('E').unwrap();

        let (path, reference_time) = find_reference(&racetrack, start, goal).unwrap();

        println!("reference: {}", reference_time);

        let cheat_candidates = find_candidates(&racetrack, &path);

        println!("candidates: {}", cheat_candidates.len());

        let mut total = 0;

        for cheat_start_pos in cheat_candidates.iter() {
            let res = find_path(&racetrack, start, goal, &path, *cheat_start_pos, 20);

            match res {
                Some((_, picoseconds)) => {
                    let saved_time = reference_time - picoseconds;
                    if saved_time >= 100 {
                        total += 1;
                    }
                }
                None => (),
            }
        }

        return total;
    });

    Ok(())
}

fn find_candidates(m: &matrix::Matrix<char>, path: &Vec<coord::Coord>) -> HashSet<coord::Coord> {
    let mut cheat_candidates: HashSet<coord::Coord> = HashSet::new();

    for p in path.iter() {
        for v in vec![UP, RIGHT, DOWN, LEFT].iter() {
            let c = p.add(*v);

            if path.contains(&c) {
                continue;
            }

            let tile = m.at_coord(c).unwrap();

            if *tile == '#' {
                cheat_candidates.insert(c);
            }
        }
    }

    return cheat_candidates;
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct Pos {
    node: coord::Coord,
    remaining: i32,
    cheat_enabled: bool,
}

fn find_reference(
    m: &matrix::Matrix<char>,
    start: coord::Coord,
    goal: coord::Coord,
) -> Option<(Vec<coord::Coord>, i32)> {
    astar(
        &start,
        |n| {
            vec![UP, RIGHT, DOWN, LEFT]
                .into_iter()
                .map(|v| n.add(v))
                .filter(|c| {
                    if !m.in_coord(*c) {
                        return false;
                    }

                    let tile = m.at_coord(*c);
                    match tile {
                        Some(t) => return *t != '#',
                        _ => return false,
                    }
                })
                .map(|c| (c, 1))
                .collect::<Vec<(coord::Coord, i32)>>()
        },
        |n| {
            let dist = n.distance(goal);
            return dist.x.abs() + dist.y.abs();
        },
        |n| n == &goal,
    )
}

fn cheated_successors(
    m: &matrix::Matrix<char>,
    reference_path: &Vec<coord::Coord>,
    n: &Pos,
    cheat_pos: coord::Coord,
    remaining: i32,
) -> Vec<(Pos, i32)> {
    let mut res: Vec<(Pos, i32)> = Vec::new();

    for v in vec![UP, RIGHT, DOWN, LEFT] {
        let next_pos = n.node.add(v);

        // skip out of bounds
        if !m.in_coord(next_pos) {
            continue;
        }

        // activate cheat
        if next_pos == cheat_pos {
            res.push((
                Pos {
                    node: next_pos,
                    remaining: remaining - 1,
                    cheat_enabled: true,
                },
                1,
            ));
            continue;
        }

        // cheat activated
        if n.cheat_enabled == true && n.remaining > 0 {
            // deactivate cheat
            if reference_path.contains(&next_pos) {
                res.push((
                    Pos {
                        node: next_pos,
                        remaining: 0,
                        cheat_enabled: false,
                    },
                    1,
                ));
            } else {
                res.push((
                    Pos {
                        node: next_pos,
                        remaining: n.remaining - 1,
                        cheat_enabled: true,
                    },
                    1,
                ));
            }
            continue;
        }

        // normal detection
        let tile = m.at_coord(next_pos).unwrap();
        if *tile != '#' {
            res.push((
                Pos {
                    node: next_pos,
                    remaining: 0,
                    cheat_enabled: false,
                },
                1,
            ));
        }
    }

    return res;
}

fn find_path(
    m: &matrix::Matrix<char>,
    start: coord::Coord,
    goal: coord::Coord,
    reference_path: &Vec<coord::Coord>,
    cheat_pos: coord::Coord,
    cheat_dur: i32,
) -> Option<(Vec<Pos>, i32)> {
    astar(
        &Pos {
            node: start,
            remaining: 0,
            cheat_enabled: false,
        },
        |n| cheated_successors(m, reference_path, n, cheat_pos, cheat_dur),
        |n| {
            let dist = n.node.distance(goal);
            return dist.x.abs() + dist.y.abs();
        },
        |n| n.node == goal,
    )
}

const LEFT: vector::Vector = vector::Vector { x: -1, y: 0 };
const RIGHT: vector::Vector = vector::Vector { x: 1, y: 0 };
const UP: vector::Vector = vector::Vector { x: 0, y: -1 };
const DOWN: vector::Vector = vector::Vector { x: 0, y: 1 };
