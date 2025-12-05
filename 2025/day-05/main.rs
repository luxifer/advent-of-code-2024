use anyhow::Result;
use common::cli;

fn main() -> Result<()> {
    let app = cli::Cli::new();
    let lines = app.content()?;

    let mut fresh_ids: Vec<(u64, u64)> = Vec::new();
    let mut ids: Vec<u64> = Vec::new();
    let mut ranges = true;

    for line in lines.lines() {
        if line.is_empty() {
            ranges = false;
            continue;
        }

        if ranges {
            let parts: Vec<u64> = line.split("-").map(|p| p.parse::<u64>().unwrap()).collect();
            fresh_ids.push((parts[0], parts[1]));
        } else {
            ids.push(line.parse::<u64>().unwrap());
        }
    }

    cli::stage(1, || -> i64 {
        let mut total = 0;

        for id in ids.iter() {
            for (start, end) in fresh_ids.iter() {
                if *id >= *start && *id <= *end {
                    total += 1;
                    break;
                }
            }
        }

        return total;
    });

    cli::stage(2, || -> i64 {
        let mut considered_fresh: Vec<(u64, u64)> = Vec::new();

        for range in fresh_ids.iter() {
            let mut tested_intervals = considered_fresh.clone();
            tested_intervals.push(*range);

            considered_fresh = merge_overlapping_intervals(&tested_intervals);
        }

        let mut total = 0;

        for (start, end) in considered_fresh.iter() {
            total += end - start + 1;
        }

        return total as i64;
    });

    Ok(())
}

fn merge_overlapping_intervals(ranges: &Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    if ranges.is_empty() {
        return vec![];
    }

    let mut sorted = ranges.clone();
    sorted.sort_by(|a, b| a.0.cmp(&b.0));

    let mut merged = vec![sorted[0].clone()];

    for b in &sorted[1..] {
        let a = merged.last_mut().unwrap();
        if a.1 < b.0 {
            merged.push(b.clone());
        } else {
            a.1 = a.1.max(b.1);
        }
    }

    return merged;
}
