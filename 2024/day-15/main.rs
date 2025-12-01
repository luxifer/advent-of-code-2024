use advent_of_code::cli;
use advent_of_code::matrix::coord;
use advent_of_code::matrix::matrix;
use advent_of_code::matrix::vector;
use anyhow::Result;
use std::collections::HashMap;

fn main() -> Result<()> {
    let app = cli::Cli::new();
    let lines = app.content()?;
    let parts: Vec<&str> = lines.split("\n\n").collect();

    let mut warehouse: matrix::Matrix<char> = matrix::Matrix::new();

    let mut cols = -1;
    let mut rows = 0;
    for line in parts[0].lines() {
        if cols == -1 {
            cols = line.len() as i32;
        }
        for char in line.chars() {
            warehouse.data.push(char);
        }
        rows += 1;
    }
    warehouse.width = cols;
    warehouse.height = rows;

    let mut movements: Vec<vector::Vector> = Vec::new();

    for line in parts[1].lines() {
        for c in line.chars() {
            match c {
                '^' => movements.push(UP),
                '>' => movements.push(RIGHT),
                'v' => movements.push(DOWN),
                '<' => movements.push(LEFT),
                _ => continue,
            }
        }
    }

    cli::stage(1, || -> i64 {
        let orig = warehouse.find('@').unwrap();
        let moveable_warehouse: &mut matrix::Matrix<char> = &mut warehouse.clone();
        let mut curr_pos = orig.clone();

        for m in movements.iter() {
            curr_pos = move_robot(curr_pos, m, moveable_warehouse);
        }

        println!("{}", moveable_warehouse);

        let mut total = 0;

        for c in moveable_warehouse.iter() {
            let tile = moveable_warehouse.at_coord(c).unwrap();

            if *tile == 'O' {
                total += 100 * c.y as i64 + c.x as i64;
            }
        }

        return total;
    });

    let mut wide_warehouse: matrix::Matrix<char> = matrix::Matrix::new();
    wide_warehouse.width = warehouse.width * 2;
    wide_warehouse.height = warehouse.height;

    for v in warehouse.data.iter() {
        match v {
            '#' => wide_warehouse.data.append(&mut vec!['#', '#']),
            'O' => wide_warehouse.data.append(&mut vec!['[', ']']),
            '.' => wide_warehouse.data.append(&mut vec!['.', '.']),
            '@' => wide_warehouse.data.append(&mut vec!['@', '.']),
            _ => continue,
        }
    }

    cli::stage(2, || -> i64 {
        let mut total = 0;
        let orig = wide_warehouse.find('@').unwrap();
        let moveable_warehouse: &mut matrix::Matrix<char> = &mut wide_warehouse.clone();
        let mut curr_pos = orig.clone();

        for d in movements.iter() {
            let moveable_pos = can_move(&moveable_warehouse, curr_pos, d);
            if moveable_pos.len() > 0 {
                curr_pos = update_warehouse(moveable_warehouse, moveable_pos, d);
            }
        }

        println!("{}", moveable_warehouse);

        for c in moveable_warehouse.iter() {
            let tile = moveable_warehouse.at_coord(c).unwrap();

            if *tile == '[' {
                total += 100 * c.y as i64 + c.x as i64;
            }
        }

        return total;
    });

    Ok(())
}

fn move_robot(
    pos: coord::Coord,
    dir: &vector::Vector,
    m: &mut matrix::Matrix<char>,
) -> coord::Coord {
    let next_pos = pos.add(*dir);

    if !m.in_coord(next_pos) {
        return pos;
    }

    let tile = m.at_coord(next_pos).unwrap();

    if *tile == '#' {
        return pos;
    }

    if *tile == '.' {
        m.set(pos, '.');
        m.set(next_pos, '@');
        return next_pos;
    }

    let mut next_box_pos = next_pos;
    let mut move_boxes = true;
    let mut boxes: Vec<coord::Coord> = Vec::new();

    loop {
        if !m.in_coord(next_box_pos) {
            break;
        }

        let next_box = m.at_coord(next_box_pos).unwrap();

        if *next_box == '.' {
            break;
        }

        if *next_box == '#' {
            move_boxes = false;
            break;
        }

        boxes.push(next_box_pos);
        next_box_pos = next_box_pos.add(*dir);
    }

    if move_boxes {
        m.set(pos, '.');
        m.set(next_pos, '@');
        for b in boxes.iter() {
            m.set(b.add(*dir), 'O');
        }
        return next_pos;
    }

    return pos;
}

fn can_move(
    m: &matrix::Matrix<char>,
    pos: coord::Coord,
    dir: &vector::Vector,
) -> Vec<coord::Coord> {
    let next_pos = pos.add(*dir);

    // cant move any further
    if !m.in_coord(next_pos) {
        return vec![];
    }

    let tile = m.at_coord(next_pos).unwrap();

    // println!("{} -> {}", tile, next_pos);

    if *tile == '#' {
        return vec![];
    }

    if *tile == '.' {
        return vec![pos];
    }

    let mut moveable_pos = can_move(m, next_pos, dir);
    if moveable_pos.len() == 0 {
        return vec![];
    }

    let other_side_dir = other_side(*tile);

    if other_side_dir.is_some() && (*dir == UP || *dir == DOWN) {
        let mut wide_parts = can_move(m, pos.add(*dir).add(other_side_dir.unwrap()), dir);
        if wide_parts.len() == 0 {
            return vec![];
        }
        moveable_pos.append(&mut wide_parts);
    }

    moveable_pos.push(pos);

    return moveable_pos;
}

fn other_side(v: char) -> Option<vector::Vector> {
    match v {
        '[' => Some(RIGHT),
        ']' => Some(LEFT),
        _ => None,
    }
}

fn update_warehouse(
    m: &mut matrix::Matrix<char>,
    moveable_pos: Vec<coord::Coord>,
    dir: &vector::Vector,
) -> coord::Coord {
    let mut already_moved: HashMap<coord::Coord, bool> = HashMap::new();
    let mut next_pos = coord::Coord::new(0, 0);

    for pos in moveable_pos.iter() {
        if already_moved.contains_key(pos) {
            continue;
        }
        already_moved.insert(*pos, true);
        let val = *m.at_coord(*pos).unwrap();
        m.set(*pos, '.');
        m.set(pos.add(*dir), val);
        if val == '@' {
            next_pos = pos.add(*dir);
        }
    }

    return next_pos;
}

const LEFT: vector::Vector = vector::Vector { x: -1, y: 0 };
const RIGHT: vector::Vector = vector::Vector { x: 1, y: 0 };
const UP: vector::Vector = vector::Vector { x: 0, y: -1 };
const DOWN: vector::Vector = vector::Vector { x: 0, y: 1 };
