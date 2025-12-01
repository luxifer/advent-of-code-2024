use advent_of_code::cli;
use advent_of_code::matrix::coord;
use advent_of_code::matrix::matrix;
use advent_of_code::matrix::vector;
use anyhow::Result;
use pathfinding::prelude::{astar, astar_bag};
use std::collections::HashSet;

fn main() -> Result<()> {
    let app = cli::Cli::new();
    let lines = app.content()?;

    let mut maze: matrix::Matrix<char> = matrix::Matrix::new();

    let mut cols = -1;
    let mut rows = 0;
    for line in lines.lines() {
        if cols == -1 {
            cols = line.len() as i32;
        }
        for char in line.chars() {
            maze.data.push(char);
        }
        rows += 1;
    }
    maze.width = cols;
    maze.height = rows;

    cli::stage(1, || -> i64 {
        let start = maze.find('S').unwrap();
        let goal = maze.find('E').unwrap();

        let result = astar(
            &Pos {
                c: start,
                dir: vector::Vector::new(1, 0),
            },
            |n| successors_weighted(&maze, n),
            |n| {
                let dist = n.c.distance(goal);
                return (dist.x.abs() + dist.y.abs()) as i64;
            },
            |n| n.c == goal,
        );

        match result {
            Some((_, score)) => {
                // path.iter().for_each(|p| println!("{}", p.c));
                // println!("{}", score);
                return score;
            }
            None => panic!("no result"),
        }
    });

    cli::stage(2, || -> i64 {
        let start = maze.find('S').unwrap();
        let goal = maze.find('E').unwrap();

        let result = astar_bag(
            &Pos {
                c: start,
                dir: vector::Vector::new(1, 0),
            },
            |n| successors_weighted(&maze, n),
            |n| {
                let dist = n.c.distance(goal);
                return (dist.x.abs() + dist.y.abs()) as i64;
            },
            |n| n.c == goal,
        );

        let mut places: HashSet<coord::Coord> = HashSet::new();
        result.map(|(solutions, _)| {
            solutions.for_each(|path| {
                path.iter().for_each(|p| _ = places.insert(p.c));
            })
        });

        return places.len() as i64;
    });

    Ok(())
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct Pos {
    c: coord::Coord,
    dir: vector::Vector,
}

fn successors_weighted(m: &matrix::Matrix<char>, n: &Pos) -> Vec<(Pos, i64)> {
    vec![
        (n.dir, 1),
        (n.dir.rotate_90_cw(), 1001),
        (n.dir.rotate_90_aw(), 1001),
        (n.dir.rotate_90_cw().rotate_90_cw(), 2001),
    ]
    .into_iter()
    .filter(|(d, _)| {
        if !m.in_coord(n.c.add(*d)) {
            return false;
        }

        let tile = m.at_coord(n.c.add(*d));
        match tile {
            Some(c) => return *c != '#',
            _ => return false,
        }
    })
    .map(|(d, score)| {
        (
            Pos {
                c: n.c.add(d),
                dir: d,
            },
            score,
        )
    })
    .collect()
}
