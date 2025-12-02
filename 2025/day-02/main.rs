use anyhow::Result;
use common::cli;

fn main() -> Result<()> {
    let app = cli::Cli::new();
    let lines = app.content()?;
    let content = lines.replace("\n", "");

    let parts: Vec<&str> = content.split(",").collect();
    let mut ranges: Vec<(i64, i64)> = Vec::new();

    for part in parts.iter() {
        let range: Vec<&str> = part.split("-").collect();
        ranges.push((
            range[0].parse::<i64>().unwrap(),
            range[1].parse::<i64>().unwrap(),
        ));
    }

    cli::stage(1, || -> i64 {
        let mut invalid_sum: i64 = 0;

        for (start, end) in ranges.iter() {
            for i in *start..=*end {
                let id = i.to_string();

                if id.len() % 2 != 0 {
                    continue;
                }

                let left = &id[..id.len() / 2];
                let right = &id[id.len() / 2..];

                if left == right {
                    invalid_sum += i;
                }
            }
        }

        return invalid_sum;
    });

    cli::stage(2, || -> i64 {
        let mut invalid_sum: i64 = 0;

        for (start, end) in ranges.iter() {
            for i in *start..=*end {
                let id = i.to_string();

                for l in 1..(id.len() / 2) + 1 {
                    let pat = &id[0..l];

                    let repeat = id.len() / pat.len();
                    let target = pat.repeat(repeat);

                    if target.len() > id.len() {
                        continue;
                    }

                    if target == id {
                        invalid_sum += i;
                        break;
                    }
                }
            }
        }

        return invalid_sum;
    });

    Ok(())
}
