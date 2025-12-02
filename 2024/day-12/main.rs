use common::matrix::coord;
use common::matrix::matrix;
use common::matrix::vector;
use anyhow::Result;
use common::cli;

fn main() -> Result<()> {
    let app = cli::Cli::new();
    let lines = app.content()?;

    let mut region: matrix::Matrix<char> = matrix::Matrix::new();

    let mut cols = -1;
    let mut rows = 0;
    for line in lines.lines() {
        if cols == -1 {
            cols = line.len() as i32;
        }
        for char in line.chars() {
            region.data.push(char);
        }
        rows += 1;
    }
    region.width = cols;
    region.height = rows;

    cli::stage(1, || -> i64 {
        let mut visited: matrix::Matrix<bool> =
            matrix::Matrix::from_size(region.width, region.height, false);
        let mut total = 0;

        for c in region.iter() {
            if *visited.at_coord(c).unwrap() {
                continue;
            }

            let label = region.at_coord(c).unwrap();

            let (a, p) = visit(&region, c, *label, &mut visited);
            total += a * p;
        }

        return total;
    });

    cli::stage(2, || -> i64 {
        let mut visited: matrix::Matrix<bool> =
            matrix::Matrix::from_size(region.width, region.height, false);
        let mut total = 0;

        for c in region.iter() {
            if *visited.at_coord(c).unwrap() {
                continue;
            }

            let label = region.at_coord(c).unwrap();

            let (a, p) = visit_corners(&region, c, *label, &mut visited);
            total += a * p;
        }

        return total;
    });

    Ok(())
}

fn visit(
    region: &matrix::Matrix<char>,
    pos: coord::Coord,
    label: char,
    visited: &mut matrix::Matrix<bool>,
) -> (i64, i64) {
    visited.set(pos, true);

    // println!("{}-> {}", label, pos);

    let dir = [UP, RIGHT, DOWN, LEFT];
    let mut area = 1;
    let mut perimeter = 0;

    for d in dir {
        let next_pos = pos.add(d);

        if !region.in_coord(next_pos) {
            perimeter += 1;
            continue;
        }

        let next_label = region.at_coord(next_pos).unwrap();

        if *next_label != label {
            perimeter += 1;
        }

        if *next_label == label && !visited.at_coord(next_pos).unwrap() {
            let (a, p) = visit(region, next_pos, label, visited);
            area += a;
            perimeter += p;
        }
    }

    return (area, perimeter);
}

fn compare(region: &matrix::Matrix<char>, a: coord::Coord, b: coord::Coord) -> bool {
    return region.in_coord(a)
        && region.in_coord(b)
        && region.at_coord(a).unwrap() == region.at_coord(b).unwrap();
}

fn count_corners(region: &matrix::Matrix<char>, pos: coord::Coord) -> i64 {
    let mut corners = 0;
    let dir = [UP, RIGHT, DOWN, LEFT];

    for d in dir {
        if !compare(region, pos, pos.add(d)) && !compare(region, pos, pos.add(d.rotate_90_cw())) {
            corners += 1;
        }

        if compare(region, pos, pos.add(d))
            && compare(region, pos, pos.add(d.rotate_90_cw()))
            && !compare(region, pos, pos.add(d).add(d.rotate_90_cw()))
        {
            corners += 1;
        }
    }

    return corners;
}

fn visit_corners(
    region: &matrix::Matrix<char>,
    pos: coord::Coord,
    label: char,
    visited: &mut matrix::Matrix<bool>,
) -> (i64, i64) {
    visited.set(pos, true);

    // println!("{}-> {}", label, pos);

    let dir = [UP, RIGHT, DOWN, LEFT];
    let mut area = 1;
    let mut sides = count_corners(region, pos);

    for d in dir {
        let next_pos = pos.add(d);

        if compare(region, pos, next_pos) && !visited.at_coord(next_pos).unwrap() {
            let (a, s) = visit_corners(region, next_pos, label, visited);
            area += a;
            sides += s;
        }
    }

    return (area, sides);
}

const LEFT: vector::Vector = vector::Vector { x: -1, y: 0 };
const RIGHT: vector::Vector = vector::Vector { x: 1, y: 0 };
const UP: vector::Vector = vector::Vector { x: 0, y: -1 };
const DOWN: vector::Vector = vector::Vector { x: 0, y: 1 };
