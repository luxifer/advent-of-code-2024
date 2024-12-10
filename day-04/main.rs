use advent_of_code::cli;
use advent_of_code::matrix::coord;
use advent_of_code::matrix::matrix;
use advent_of_code::matrix::vector;
use anyhow::Result;
use std::collections::HashMap;

fn main() -> Result<()> {
    let app = cli::Cli::new();

    let mut word_search: matrix::Matrix<char> = matrix::Matrix::new();
    let lines = app.content()?;

    let mut cols = -1;
    let mut rows = 0;
    for line in lines.lines() {
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
    Ok(())
}

fn search_xmas(m: &matrix::Matrix<char>, orig: coord::Coord, dir: vector::Vector) -> bool {
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

fn search_cross_mas(m: &matrix::Matrix<char>, orig: coord::Coord) -> bool {
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

const LEFT: vector::Vector = vector::Vector { x: -1, y: 0 };
const RIGHT: vector::Vector = vector::Vector { x: 1, y: 0 };
const UP: vector::Vector = vector::Vector { x: 0, y: -1 };
const DOWN: vector::Vector = vector::Vector { x: 0, y: 1 };
const UP_LEFT: vector::Vector = UP.add(LEFT);
const UP_RIGHT: vector::Vector = UP.add(RIGHT);
const DOWN_LEFT: vector::Vector = DOWN.add(LEFT);
const DOWN_RIGHT: vector::Vector = DOWN.add(RIGHT);
