use anyhow::Result;
use common::cli;
use common::matrix::coord;
use common::matrix::matrix;
use std::collections::HashMap;

fn main() -> Result<()> {
    let app = cli::Cli::new();

    let mut antena_map: matrix::Matrix<char> = matrix::Matrix::new();
    let lines = app.content()?;

    let mut cols = -1;
    let mut rows = 0;
    for line in lines.lines() {
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

    let mut antenas: HashMap<char, Vec<coord::Coord>> = HashMap::new();

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

    cli::stage(1, || -> i64 {
        let antinodes = search_antinodes(&antenas, &antena_map);
        let mut antinode_map = antena_map.clone();
        antinodes.iter().for_each(|p| antinode_map.set(*p, '#'));

        return antinodes.len() as i64;
    });

    cli::stage(2, || -> i64 {
        let antinodes = search_antinodes_harmonics(&antenas, &antena_map);
        let mut antinode_map = antena_map.clone();
        antinodes.iter().for_each(|p| antinode_map.set(*p, '#'));
        return antinodes.len() as i64;
    });

    Ok(())
}

fn search_antinodes_harmonics(
    antenas: &HashMap<char, Vec<coord::Coord>>,
    antena_map: &matrix::Matrix<char>,
) -> Vec<coord::Coord> {
    let mut antinodes: Vec<coord::Coord> = Vec::new();

    for (_, coords) in antenas.iter() {
        for antena in coords.iter() {
            for other_antena in coords.iter() {
                let distance = antena.distance(*other_antena);

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

fn search_antinodes(
    antenas: &HashMap<char, Vec<coord::Coord>>,
    antena_map: &matrix::Matrix<char>,
) -> Vec<coord::Coord> {
    let mut antinodes: Vec<coord::Coord> = Vec::new();

    for (_, coords) in antenas.iter() {
        for antena in coords.iter() {
            for other_antena in coords.iter() {
                let distance = antena.distance(*other_antena);

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
