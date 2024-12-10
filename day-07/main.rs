use advent_of_code::cli;
use anyhow::Result;
use itertools::{repeat_n, Itertools};
use std::fmt;
use std::fmt::Write;

fn main() -> Result<()> {
    let app = cli::Cli::new();

    let mut equations: Vec<Equation> = Vec::new();
    let lines = app.content()?;

    for line in lines.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        let test_value = parts[0].parse::<u64>().unwrap();
        let numbers: Vec<u64> = parts[1]
            .split(" ")
            .map(|p| p.parse::<u64>().unwrap())
            .collect();

        equations.push(Equation::from(test_value, numbers));
    }

    let mut total = 0;

    for equation in equations.iter() {
        if compute_combination(
            equation.test_value,
            equation.numbers.clone(),
            vec![Operator::Addition, Operator::Multiplication],
        ) {
            total += equation.test_value;
        }
    }

    println!("answer: {}", total);
    total = 0;

    for equation in equations.iter() {
        if compute_combination(
            equation.test_value,
            equation.numbers.clone(),
            vec![
                Operator::Addition,
                Operator::Multiplication,
                Operator::Concatenation,
            ],
        ) {
            total += equation.test_value;
        }
    }

    println!("answer: {}", total);
    Ok(())
}

struct Equation {
    test_value: u64,
    numbers: Vec<u64>,
}

impl Equation {
    fn from(test_value: u64, numbers: Vec<u64>) -> Equation {
        Equation {
            test_value,
            numbers,
        }
    }
}

fn compute_combination(expected: u64, inputs: Vec<u64>, operators: Vec<Operator>) -> bool {
    let permutations: Vec<_> = repeat_n(operators.clone(), inputs.len() - 1)
        .multi_cartesian_product()
        .collect();

    for permutation in permutations.iter() {
        let mut total = inputs[0];
        let mut buf = String::new();
        write!(buf, "{}: {}", expected, inputs[0]).unwrap();

        for i in 0..permutation.len() {
            write!(buf, " {} {}", permutation[i], inputs[i + 1]).unwrap();

            if permutation[i] == Operator::Addition {
                total += inputs[i + 1];
            }

            if permutation[i] == Operator::Multiplication {
                total *= inputs[i + 1];
            }

            if permutation[i] == Operator::Concatenation {
                let concatenation = total.to_string() + &inputs[i + 1].to_string();
                total = concatenation.parse::<u64>().unwrap();
            }

            if total > expected {
                break;
            }
        }

        if total == expected {
            // println!("{} = {}", buf, total);
            return true;
        }
    }

    return false;
}

#[derive(Clone, PartialEq, Debug)]
enum Operator {
    Addition,
    Multiplication,
    Concatenation,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operator::Addition => write!(f, "+"),
            Operator::Multiplication => write!(f, "*"),
            Operator::Concatenation => write!(f, "||"),
        }
    }
}
