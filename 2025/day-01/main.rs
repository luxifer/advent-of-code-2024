use advent_of_code::cli;
use anyhow::Result;

fn main() -> Result<()> {
    let app = cli::Cli::new();
    let lines = app.content()?;
    let mut rotations: Vec<(Rotation, i32)> = Vec::new();

    for line in lines.lines() {
        if line.chars().nth(0).unwrap() == 'L' {
            rotations.push((Rotation::Left, line[1..].parse::<i32>().unwrap()));
        } else {
            rotations.push((Rotation::Right, line[1..].parse::<i32>().unwrap()));
        }
    }

    cli::stage(1, || -> i64 {
        let mut dial = 50;
        let mut at_zero: i64 = 0;

        for (dir, val) in rotations.iter() {
            if *dir == Rotation::Right {
                for _ in 0..*val {
                    dial += 1;
                    if dial > 99 {
                        dial = 0;
                    }
                }
            } else {
                for _ in 0..*val {
                    dial -= 1;
                    if dial < 0 {
                        dial = 99;
                    }
                }
            }

            if dial == 0 {
                at_zero += 1;
            }
        }

        return at_zero;
    });

    cli::stage(2, || -> i64 {
        let mut dial = 50;
        let mut at_zero: i64 = 0;

        for (dir, val) in rotations.iter() {
            if *dir == Rotation::Right {
                for _ in 0..*val {
                    dial += 1;
                    if dial > 99 {
                        dial = 0;
                    }

                    if dial == 0 {
                        at_zero += 1;
                    }
                }
            } else {
                for _ in 0..*val {
                    dial -= 1;
                    if dial < 0 {
                        dial = 99;
                    }

                    if dial == 0 {
                        at_zero += 1;
                    }
                }
            }
        }

        return at_zero;
    });

    Ok(())
}

#[derive(Clone, PartialEq, Debug)]
enum Rotation {
    Left,
    Right,
}
