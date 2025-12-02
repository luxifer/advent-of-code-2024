use anyhow::Result;
use common::cli;

fn main() -> Result<()> {
    let app = cli::Cli::new();

    let mut ordering: Vec<[i64; 2]> = Vec::new();
    let mut updates: Vec<Vec<i64>> = Vec::new();
    let lines = app.content()?;

    for line in lines.lines() {
        if line.contains("|") {
            let order: Vec<i64> = line.split("|").map(|p| p.parse::<i64>().unwrap()).collect();
            ordering.push([order[0], order[1]]);
        }

        if line.contains(",") {
            let update: Vec<i64> = line.split(",").map(|p| p.parse::<i64>().unwrap()).collect();
            updates.push(update);
        }
    }

    cli::stage(1, || -> i64 {
        return find_ordered(&ordering, updates.clone(), true);
    });

    cli::stage(2, || -> i64 {
        return find_ordered(&ordering, updates.clone(), false);
    });

    Ok(())
}

fn find_ordered(ordering: &Vec<[i64; 2]>, mut updates: Vec<Vec<i64>>, want: bool) -> i64 {
    let mut total = 0;

    for update in updates.iter_mut() {
        let mut valid = true;
        let mut ordered = false;

        while !ordered {
            ordered = true;
            for rule in ordering.iter() {
                let left_index = update.iter().position(|p| *p == rule[0]);
                let right_index = update.iter().position(|p| *p == rule[1]);

                if left_index.is_some() && right_index.is_some() {
                    if right_index.unwrap() < left_index.unwrap() {
                        valid = false;
                        update.swap(left_index.unwrap(), right_index.unwrap());
                        ordered = false;
                        break;
                    }
                }
            }
        }

        if valid && want {
            total += update.get(update.len() / 2).unwrap();
        }

        if !valid && !want {
            total += update.get(update.len() / 2).unwrap();
        }
    }

    return total;
}
