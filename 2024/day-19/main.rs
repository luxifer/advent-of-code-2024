use anyhow::Result;
use common::cli;
use std::collections::HashMap;

fn main() -> Result<()> {
    let app = cli::Cli::new();
    let lines = app.content()?;

    let parts: Vec<&str> = lines.split("\n\n").collect();

    let towels: Vec<&str> = parts[0].split(", ").collect();

    let mut patterns: Vec<&str> = Vec::new();

    for pattern in parts[1].lines() {
        patterns.push(pattern);
    }

    cli::stage(1, || -> i64 {
        let mut total = 0;
        for pattern in patterns.iter() {
            if find_pattern(&pattern, &towels) {
                total += 1;
            }
        }

        return total;
    });

    cli::stage(2, || -> i64 {
        let mut total = 0;
        let mut finder = PatternFinder::new();

        for pattern in patterns.iter() {
            total += finder.count_pattern(&pattern, &towels);
        }

        return total;
    });

    Ok(())
}

fn find_pattern(expected: &str, towels: &Vec<&str>) -> bool {
    if expected.len() == 0 {
        return true;
    }

    for towel in towels.iter() {
        if expected.starts_with(towel) {
            let is_found = find_pattern(&expected[towel.len()..], towels);
            if is_found {
                return true;
            }
        }
    }

    return false;
}

struct PatternFinder<'a> {
    cache: HashMap<&'a str, i64>,
}

impl<'a> PatternFinder<'a> {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    fn count_pattern(&mut self, expected: &'a str, towels: &Vec<&str>) -> i64 {
        if self.cache.contains_key(expected) {
            return *self.cache.get(expected).unwrap();
        }

        if expected.len() == 0 {
            return 1;
        }

        let mut ways = 0;
        for towel in towels.iter() {
            if towel.len() > expected.len() {
                continue;
            }

            if expected.starts_with(towel) {
                ways += self.count_pattern(&expected[towel.len()..], towels);
            }
        }

        self.cache.insert(&expected, ways);

        return ways;
    }
}
