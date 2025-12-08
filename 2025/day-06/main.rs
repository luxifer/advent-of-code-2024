use anyhow::Result;
use common::cli;

fn main() -> Result<()> {
    let app = cli::Cli::new();
    let lines = app.content()?;

    let mut numbers: Vec<Vec<i64>> = Vec::new();
    let mut operators: Vec<Operator> = Vec::new();
    let mut problem_len = 0;
    let mut inputs: Vec<Vec<char>> = Vec::new();

    for line in lines.lines() {
        problem_len = line.len();
        inputs.push(line.chars().collect());
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

    let mut cur_prob: Vec<i64> = Vec::new();
    let mut problems: Vec<Vec<i64>> = Vec::new();

    for x in (0..problem_len).rev() {
        let mut acc: String = String::new();
        for y in 0..inputs.len() {
            acc.push_str(inputs[y][x].to_string().as_str());
        }

        acc = acc.trim().to_string();

        if acc == "" {
            problems.push(cur_prob.clone());
            cur_prob = Vec::new();
            continue;
        }

        if acc.chars().last().unwrap() == '+' || acc.chars().last().unwrap() == '*' {
            acc = acc[..acc.len() - 1].to_string();
        }

        cur_prob.push(acc.trim().parse::<i64>().unwrap());
    }

    problems.push(cur_prob.clone());

    cli::stage(2, || -> i64 {
        let mut new_operators = operators.clone();
        new_operators.reverse();
        let mut result: Vec<i64> = Vec::new();

        for i in 0..problems.len() {
            let res =
                problems[i][1..]
                    .iter()
                    .fold(problems[i][0], |acc, e| match new_operators[i] {
                        Operator::Addition => acc + e,
                        Operator::Multiplication => acc * e,
                    });
            result.push(res);
        }

        return result.iter().fold(0, |acc, e| acc + e);
    });

    Ok(())
}

#[derive(Clone, PartialEq, Debug)]
enum Operator {
    Addition,
    Multiplication,
}
