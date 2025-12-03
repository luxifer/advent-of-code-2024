use anyhow::Result;
use common::cli;

fn main() -> Result<()> {
    let app = cli::Cli::new();
    let lines = app.content()?;

    let banks: Vec<&str> = lines.lines().collect();

    cli::stage(1, || -> i64 {
        let mut total_joltage: i64 = 0;

        for bank in banks.iter() {
            total_joltage += find_joltage(&bank, 2);
        }

        return total_joltage;
    });

    cli::stage(2, || -> i64 {
        let mut total_joltage: i64 = 0;

        for bank in banks.iter() {
            total_joltage += find_joltage(&bank, 12);
        }

        return total_joltage;
    });

    Ok(())
}

fn find_joltage(bank: &str, size: usize) -> i64 {
    let mut next_index: usize = 0;
    let mut joltage = String::new();

    for end in (0..size).rev() {
        let mut max = 0;
        for i in next_index..bank.len() - end {
            let c = bank.chars().nth(i).unwrap();
            let d = c.to_digit(10).unwrap();

            if d > max {
                max = d;
                next_index = i + 1;
            }
        }

        joltage.push_str(&max.to_string());
    }

    return joltage.parse::<i64>().unwrap();
}
