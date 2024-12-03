use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let input = std::env::args().nth(1).expect("missing input");

    let mut programs: Vec<String> = Vec::new();

    if let Ok(lines) = read_lines(input) {
        for line in lines.flatten() {
            programs.push(line.clone());
        }
    }

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut res = 0;

    for program in programs.iter() {
        for (_, [left, right]) in re.captures_iter(program).map(|c| c.extract()) {
            res += left.parse::<i32>().unwrap() * right.parse::<i32>().unwrap();
        }
    }

    println!("answer: {}", res);

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    let mut enabled = true;
    let mut res = 0;

    for program in programs.iter() {
        for m in re.captures_iter(program) {
            let Some(ins) = m.get(0) else { return };
            if ins.as_str() == "don't()" {
                enabled = false;
                println!("disabled");
            }

            if ins.as_str() == "do()" {
                enabled = true;
                println!("enabled");
            }

            if enabled {
                let Some(left) = m.get(1) else {
                    continue;
                };
                let Some(right) = m.get(2) else {
                    continue;
                };
                res +=
                    left.as_str().parse::<i32>().unwrap() * right.as_str().parse::<i32>().unwrap();
            }
        }
    }

    println!("answer: {}", res);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
