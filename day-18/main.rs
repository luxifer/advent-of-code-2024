use advent_of_code::cli;
use advent_of_code::matrix::coord;
use advent_of_code::matrix::matrix;
use advent_of_code::matrix::vector;
use anyhow::Result;
use pathfinding::prelude::astar;

fn main() -> Result<()> {
    let app = cli::Cli::new();
    let lines = app.content()?;
    let mut bytes: Vec<coord::Coord> = Vec::new();

    for line in lines.lines() {
        let pos: Vec<i32> = line.split(",").map(|v| v.parse::<i32>().unwrap()).collect();
        bytes.push(coord::Coord::new(pos[0], pos[1]));
    }

    let size = 71;
    let drop_count = 1024;

    let memory_cell: matrix::Matrix<char> = matrix::Matrix::from_size(size, size, '.');

    cli::stage(1, || -> i64 {
        let mut new_memory_cell = memory_cell.clone();
        for i in 0..drop_count {
            new_memory_cell.set(bytes[i], '#');
        }

        // println!("{}", new_memory_cell);

        let start = coord::Coord::new(0, 0);
        let goal = coord::Coord::new(size - 1, size - 1);

        let result = find_path(&new_memory_cell, start, goal);

        match result {
            Some((path, _)) => {
                // path.iter().for_each(|p| println!("{}", p.c));
                // println!("{}", score);
                return path.len() as i64 - 1;
            }
            None => panic!("no result"),
        }
    });

    cli::stage(2, || -> i64 {
        let mut new_memory_cell = memory_cell.clone();
        let start = coord::Coord::new(0, 0);
        let goal = coord::Coord::new(size - 1, size - 1);

        for c in bytes.iter() {
            new_memory_cell.set(*c, '#');
            // println!("{}", new_memory_cell);

            let result = find_path(&new_memory_cell, start, goal);

            if result.is_none() {
                println!("{}", c);
                break;
            }
        }

        return 0;
    });

    Ok(())
}

fn find_path(
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

const LEFT: vector::Vector = vector::Vector { x: -1, y: 0 };
const RIGHT: vector::Vector = vector::Vector { x: 1, y: 0 };
const UP: vector::Vector = vector::Vector { x: 0, y: -1 };
const DOWN: vector::Vector = vector::Vector { x: 0, y: 1 };
