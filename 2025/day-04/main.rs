use anyhow::Result;
use common::cli;
use common::matrix::coord;
use common::matrix::matrix;
use common::matrix::vector;

fn main() -> Result<()> {
    let app = cli::Cli::new();
    let lines = app.content()?;
    let mut diagram: matrix::Matrix<char> = matrix::Matrix::new();

    let mut cols = -1;
    let mut rows = 0;
    for line in lines.lines() {
        if cols == -1 {
            cols = line.len() as i32;
        }
        for char in line.chars() {
            diagram.data.push(char);
        }
        rows += 1;
    }
    diagram.width = cols;
    diagram.height = rows;

    cli::stage(1, || -> i64 {
        let mut accessible = 0;

        for c in diagram.iter() {
            let val = diagram.at_coord(c).unwrap();
            if *val != '@' {
                continue;
            }

            if is_accessible(c, &diagram) {
                accessible += 1;
            }
        }

        return accessible;
    });

    cli::stage(2, || -> i64 {
        let mut accessible = 0;
        let mut cur_diag = diagram.clone();

        loop {
            let (new_diag, inner) = walk_diag(&cur_diag);
            if inner == 0 {
                break;
            }

            accessible += inner;
            cur_diag = new_diag;
        }

        return accessible;
    });

    Ok(())
}

fn walk_diag(diagram: &matrix::Matrix<char>) -> (matrix::Matrix<char>, i64) {
    let mut accessible = 0;
    let mut new_diag = diagram.clone();

    for c in diagram.iter() {
        let val = diagram.at_coord(c).unwrap();
        if *val != '@' {
            continue;
        }

        if is_accessible(c, &diagram) {
            accessible += 1;
            new_diag.set(c, '.');
        }
    }

    return (new_diag, accessible);
}

fn is_accessible(c: coord::Coord, diagram: &matrix::Matrix<char>) -> bool {
    let directions = [
        UP, UP_RIGHT, RIGHT, DOWN_RIGHT, DOWN, DOWN_LEFT, LEFT, UP_LEFT,
    ];

    let mut adjacent = 0;
    for d in directions {
        let cur = c.add(d);

        if !diagram.in_coord(cur) {
            continue;
        }

        let val = diagram.at_coord(cur).unwrap();
        if *val == '@' {
            adjacent += 1;
        }
    }

    return adjacent < 4;
}

const LEFT: vector::Vector = vector::Vector { x: -1, y: 0 };
const RIGHT: vector::Vector = vector::Vector { x: 1, y: 0 };
const UP: vector::Vector = vector::Vector { x: 0, y: -1 };
const DOWN: vector::Vector = vector::Vector { x: 0, y: 1 };
const UP_LEFT: vector::Vector = UP.add(LEFT);
const UP_RIGHT: vector::Vector = UP.add(RIGHT);
const DOWN_LEFT: vector::Vector = DOWN.add(LEFT);
const DOWN_RIGHT: vector::Vector = DOWN.add(RIGHT);
