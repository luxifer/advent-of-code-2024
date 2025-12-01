use advent_of_code::cli;
use anyhow::Result;

fn main() -> Result<()> {
    let app = cli::Cli::new();

    let mut reports: Vec<Vec<i32>> = Vec::new();
    let lines = app.content()?;

    for line in lines.lines() {
        let parts: Vec<&str> = line.split(" ").collect();
        let levels: Vec<i32> = parts.iter().map(|p| p.parse::<i32>().unwrap()).collect();
        reports.push(levels);
    }

    cli::stage(1, || -> i64 {
        let mut safe: i64 = 0;

        for levels in reports.iter() {
            if is_safe(levels.clone()) {
                safe += 1;
            }
        }

        return safe;
    });

    cli::stage(2, || -> i64 {
        let mut safe: i64 = 0;

        for levels in reports.iter() {
            let mut any_safe = false;

            for i in 0..levels.len() {
                let mut fixed: Vec<i32> = levels.to_vec();
                fixed.remove(i);

                if is_safe(fixed) {
                    any_safe = true;
                    break;
                }
            }

            if any_safe {
                safe += 1;
            }
        }

        return safe;
    });

    Ok(())
}

fn is_safe(levels: Vec<i32>) -> bool {
    let mut prev: i32 = -1;
    let mut is_safe: bool = true;
    let mut dir = 0;

    for level in levels.iter() {
        let mut new_dir = 0;
        if prev != -1 {
            let diff = prev.abs_diff(*level);
            if diff > 3 {
                is_safe = false;
                break;
            }

            if prev == *level {
                is_safe = false;
                break;
            }

            if prev < *level {
                new_dir = 1;
            } else {
                new_dir = -1
            }
        }

        if dir != 0 {
            if dir != new_dir {
                is_safe = false;
                break;
            }
        }

        prev = *level;
        dir = new_dir;
    }

    return is_safe;
}
