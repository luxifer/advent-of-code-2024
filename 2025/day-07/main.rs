use anyhow::Result;
use common::cli;
use std::collections::{HashMap, HashSet};

fn main() -> Result<()> {
    let app = cli::Cli::new();
    let lines = app.content()?;
    let manifold: Vec<&str> = lines.lines().collect();

    cli::stage(1, || -> i64 {
        let mut total = 0;
        let mut beam_pos = HashSet::from([manifold[0].find('S').unwrap()]);

        for i in 1..manifold.len() {
            let mut new_pos: HashSet<usize> = HashSet::new();
            let mut existing: HashSet<usize> = HashSet::new();
            let mut splitted: HashSet<usize> = HashSet::new();

            for p in &beam_pos {
                let c = manifold[i].chars().nth(*p).unwrap(); // This is the current character at position p
                match c {
                    '.' => {
                        new_pos.insert(*p);
                        existing.insert(*p);
                    }
                    '^' => {
                        let left = *p as i32 - 1;
                        let right = *p as i32 + 1;

                        if left >= 0 {
                            new_pos.insert(left as usize);
                            splitted.insert(*p);
                        }

                        if right <= manifold[i].len() as i32 {
                            new_pos.insert(right as usize);
                            splitted.insert(*p);
                        }
                    }
                    _ => continue,
                }
            }

            let diff: Vec<&usize> = splitted.difference(&existing).collect();

            total += diff.len();
            beam_pos = new_pos.clone();
        }

        return total as i64;
    });

    cli::stage(2, || -> i64 {
        let start_pos = manifold[0].find('S').unwrap();
        let mut finder = TachyonPathFinder {
            visited: HashMap::new(),
        };

        return finder.find_timeline(&manifold, start_pos, 1);
    });

    Ok(())
}

struct TachyonPathFinder {
    visited: HashMap<(usize, usize), i64>,
}

impl TachyonPathFinder {
    fn find_timeline(&mut self, manifold: &Vec<&str>, pos: usize, line: usize) -> i64 {
        let mut total = 0;

        if line >= manifold.len() {
            return 1;
        }

        if self.visited.contains_key(&(line, pos)) {
            return *self.visited.get(&(line, pos)).unwrap();
        }

        let c = manifold[line].chars().nth(pos).unwrap();
        match c {
            '.' => total += self.find_timeline(manifold, pos, line + 1),
            '^' => {
                let left = pos as i32 - 1;
                let right = pos as i32 + 1;

                if left >= 0 {
                    total += self.find_timeline(manifold, left as usize, line + 1);
                }

                if right <= manifold[line].len() as i32 {
                    total += self.find_timeline(manifold, right as usize, line + 1);
                }
            }
            _ => (),
        }

        self.visited.insert((line, pos), total); // Store the result in visited map to av

        return total;
    }
}
