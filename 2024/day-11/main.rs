use advent_of_code::cli;
use anyhow::Result;
use std::collections::HashMap;

fn main() -> Result<()> {
    let app = cli::Cli::new();
    let lines = app.content()?;
    let mut stones: Vec<i128> = Vec::new();

    for line in lines.lines() {
        line.split(" ")
            .map(|p| p.parse::<i128>().unwrap())
            .for_each(|p| stones.push(p));
    }

    cli::stage(1, || -> i64 {
        let mut blinker = Blinker::new();
        let mut total = 0;

        for stone in stones.clone() {
            total += blinker.blink_recurse(stone, 25);
        }

        return total;
    });

    cli::stage(2, || -> i64 {
        let mut blinker = Blinker::new();
        let mut total = 0;

        for stone in stones.clone() {
            total += blinker.blink_recurse(stone, 75);
        }

        return total;
    });

    Ok(())
}

fn stone_rule(stone: i128) -> Vec<i128> {
    if stone == 0 {
        return vec![1];
    }

    if stone.to_string().len() % 2 == 0 {
        let mut left_part = stone.to_string();
        let right_part = left_part.split_off(left_part.len() / 2);

        return vec![
            left_part.parse::<i128>().unwrap(),
            right_part.parse::<i128>().unwrap(),
        ];
    }

    return vec![stone * 2024];
}

#[derive(Eq, Hash, PartialEq)]
struct CacheKey {
    stone: i128,
    times: i32,
}

struct Blinker {
    cache: HashMap<CacheKey, i64>,
}

impl Blinker {
    fn new() -> Self {
        Blinker {
            cache: HashMap::new(),
        }
    }

    fn blink_recurse(&mut self, stone: i128, times: i32) -> i64 {
        if times == 0 {
            return 1;
        }

        let key = CacheKey { stone, times };
        if self.cache.contains_key(&key) {
            return *self.cache.get(&key).unwrap();
        }

        let new_stones = stone_rule(stone);
        let mut total = 0;

        for s in new_stones.iter() {
            total += self.blink_recurse(*s, times - 1);
        }

        self.cache.insert(key, total);

        return total;
    }
}
