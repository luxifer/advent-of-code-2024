use anyhow::Result;
use common::cli;
use ndarray::{Array, Array2, Axis};

fn main() -> Result<()> {
    let app = cli::Cli::new();
    let lines = app.content()?;

    let mut numbers: Vec<Vec<i64>> = Vec::new();
    let mut operators: Vec<Operator> = Vec::new();
    let mut problem_len = 0;

    for line in lines.lines() {
        problem_len = line.len();
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts[0] == "+" || parts[0] == "*" {
            for op in parts {
                if op == "+" {
                    operators.push(Operator::Addition);
                }

                if op == "*" {
                    operators.push(Operator::Multiplication);
                }
            }
        } else {
            numbers.push(parts.iter().map(|p| p.parse::<i64>().unwrap()).collect());
        }
    }

    cli::stage(1, || -> i64 {
        let mut result: Vec<i64> = numbers[0].clone();

        for line in numbers[1..].iter() {
            for i in 0..result.len() {
                match operators[i] {
                    Operator::Addition => result[i] += line[i],
                    Operator::Multiplication => result[i] *= line[i],
                }
            }
        }

        return result.iter().fold(0, |acc, e| acc + e);
    });

    let mut problem: Array2<char> = Array2::default((0, problem_len));

    for line in lines.lines() {
        if line.contains("+") || line.contains("*") {
            continue;
        }

        let chars: Vec<char> = line.chars().collect();
        problem.append(
            Axis(0),
            Array::from_shape_vec((1, line.len()), chars)
                .unwrap()
                .view(),
        )?;
    }

    println!("{}", problem);

    Ok(())
}

#[derive(Clone, PartialEq, Debug)]
enum Operator {
    Addition,
    Multiplication,
}
