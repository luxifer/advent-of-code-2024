use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let input = std::env::args().nth(1).expect("missing input");

    let mut ordering: Vec<[i32; 2]> = Vec::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();

    if let Ok(lines) = read_lines(input) {
        for line in lines.flatten() {
            if line.contains("|") {
                let order: Vec<i32> = line.split("|").map(|p| p.parse::<i32>().unwrap()).collect();
                ordering.push([order[0], order[1]]);
            }

            if line.contains(",") {
                let update: Vec<i32> = line.split(",").map(|p| p.parse::<i32>().unwrap()).collect();
                updates.push(update);
            }
        }
    }

    let mut total_correct = 0;
    let mut total_incorrect = 0;

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

        if valid {
            total_correct += update.get(update.len() / 2).unwrap();
        } else {
            total_incorrect += update.get(update.len() / 2).unwrap();
        }
    }

    println!("answer: {}", total_correct);
    println!("answer: {}", total_incorrect);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
