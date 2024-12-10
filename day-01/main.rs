use advent_of_code::cli;
use anyhow::Result;

fn main() -> Result<()> {
    let app = cli::Cli::new();

    let mut col1: Vec<i32> = Vec::new();
    let mut col2: Vec<i32> = Vec::new();
    let lines = app.content()?;

    for line in lines.lines() {
        let parts: Vec<&str> = line.split("   ").collect();

        col1.push(parts[0].parse::<i32>().unwrap());
        col2.push(parts[1].parse::<i32>().unwrap());
    }

    col1.sort();
    col2.sort();

    let mut dist: u32 = 0;

    for i in 0..col1.len() {
        dist += col1[i].abs_diff(col2[i]);
    }

    println!("answer: {}", dist);

    let mut sim: i32 = 0;

    for left in col1.iter() {
        let found = col2.iter().filter(|val| val == &left).count() as i32;
        sim += left * found;
    }

    println!("answer: {}", sim);
    Ok(())
}
