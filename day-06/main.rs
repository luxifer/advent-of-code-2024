use advent_of_code::matrix::coord;
use advent_of_code::matrix::matrix;
use advent_of_code::matrix::vector;
use advent_of_code::utils;
use std::collections::HashMap;

fn main() {
    let input = std::env::args().nth(1).expect("missing input");

    let mut search_map: matrix::Matrix<char> = matrix::Matrix::new();

    if let Ok(lines) = utils::read_lines(input) {
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
    let dir = vector::Vector { x: 0, y: -1 };

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

fn move_guard(
    m: &matrix::Matrix<char>,
    mut guard: coord::Coord,
    mut dir: vector::Vector,
) -> matrix::Matrix<char> {
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

fn detect_loop(mut m: matrix::Matrix<char>, orig: coord::Coord, mut dir: vector::Vector) -> bool {
    let mut visited: HashMap<coord::Coord, Vec<vector::Vector>> = HashMap::new();
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
