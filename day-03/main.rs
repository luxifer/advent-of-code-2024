use advent_of_code::cli;
use anyhow::Result;
use regex::Regex;

fn main() -> Result<()> {
    let app = cli::Cli::new();

    let mut programs: Vec<String> = Vec::new();
    let lines = app.content()?;

    for line in lines.lines() {
        programs.push(line.to_string());
    }

    cli::stage(1, || -> i64 {
        let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
        let mut res: i64 = 0;

        for program in programs.iter() {
            for (_, [left, right]) in re.captures_iter(program).map(|c| c.extract()) {
                res += left.parse::<i64>().unwrap() * right.parse::<i64>().unwrap();
            }
        }

        return res;
    });

    cli::stage(2, || -> i64 {
        let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
        let mut enabled = true;
        let mut res: i64 = 0;

        for program in programs.iter() {
            for m in re.captures_iter(program) {
                let Some(ins) = m.get(0) else {
                    continue;
                };
                if ins.as_str() == "don't()" {
                    enabled = false;
                    // println!("disabled");
                }

                if ins.as_str() == "do()" {
                    enabled = true;
                    // println!("enabled");
                }

                if enabled {
                    let Some(left) = m.get(1) else {
                        continue;
                    };
                    let Some(right) = m.get(2) else {
                        continue;
                    };
                    res += left.as_str().parse::<i64>().unwrap()
                        * right.as_str().parse::<i64>().unwrap();
                }
            }
        }

        return res;
    });

    Ok(())
}
