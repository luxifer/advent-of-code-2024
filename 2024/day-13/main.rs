use advent_of_code::cli;
use anyhow::Result;
use regex::Regex;

fn main() -> Result<()> {
    let app = cli::Cli::new();
    let lines = app.content()?;

    let mut claw_machines: Vec<ClawMachine> = Vec::new();

    let re = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
    )?;

    for (_, [xa, ya, xb, yb, xp, yp]) in re.captures_iter(&lines).map(|c| c.extract()) {
        claw_machines.push(ClawMachine {
            button_a: (xa.parse::<i128>()?, ya.parse::<i128>()?),
            button_b: (xb.parse::<i128>()?, yb.parse::<i128>()?),
            prize: (xp.parse::<i128>()?, yp.parse::<i128>()?),
        });
    }

    cli::stage(1, || -> i64 {
        let mut total = 0;

        for m in claw_machines.iter() {
            let res = solve(m);

            if res.is_none() {
                continue;
            }

            let (a, b) = res.unwrap();

            if a > 100 || b > 100 {
                // println!("to many presses, {}xA {}xB", a, b);
                continue;
            }

            if a < 0 || b < 0 {
                // println!("cannot have negative answer, {}xA {}xB", a, b);
                continue;
            }

            // println!("{} -> {}xA, {}xB", m.prize, a, b);

            total += a * 3 + b;
        }

        return total as i64;
    });

    cli::stage(2, || -> i64 {
        let mut total = 0;

        for m in claw_machines.iter() {
            let res = solve_factor(m, 10000000000000);

            if res.is_none() {
                continue;
            }

            let (a, b) = res.unwrap();

            if a < 0 || b < 0 {
                // println!("cannot have negative answer, {}xA {}xB", a, b);
                continue;
            }

            // println!("{} -> {}xA, {}xB", m.prize, a, b);

            total += a * 3 + b;
        }

        return total as i64;
    });

    Ok(())
}

struct ClawMachine {
    button_a: (i128, i128),
    button_b: (i128, i128),
    prize: (i128, i128),
}

fn solve(m: &ClawMachine) -> Option<(i128, i128)> {
    // println!("A: {}", m.button_a);
    // println!("B: {}", m.button_b);
    // println!("Prize: {}", m.prize);

    let determinant = m.button_a.0 * m.button_b.1 - m.button_a.1 * m.button_b.0;
    // println!("{}", determinant);

    if determinant == 0 {
        // println!("no solutions");
        return None;
    }

    let a = (m.prize.0 * m.button_b.1 - m.prize.1 * m.button_b.0) / determinant;
    let b = (m.prize.1 * m.button_a.0 - m.prize.0 * m.button_a.1) / determinant;

    if a * m.button_a.0 + b * m.button_b.0 != m.prize.0 {
        return None;
    }

    if a * m.button_a.1 + b * m.button_b.1 != m.prize.1 {
        return None;
    }

    return Some((a, b));
}

fn solve_factor(m: &ClawMachine, factor: i128) -> Option<(i128, i128)> {
    let prize0 = m.prize.0 + factor;
    let prize1 = m.prize.1 + factor;

    let determinant = m.button_a.0 * m.button_b.1 - m.button_a.1 * m.button_b.0;
    // println!("{}", determinant);

    if determinant == 0 {
        // println!("no solutions");
        return None;
    }

    let a = (prize0 * m.button_b.1 - prize1 * m.button_b.0) / determinant;
    let b = (prize1 * m.button_a.0 - prize0 * m.button_a.1) / determinant;

    if a * m.button_a.0 + b * m.button_b.0 != prize0 {
        return None;
    }

    if a * m.button_a.1 + b * m.button_b.1 != prize1 {
        return None;
    }

    return Some((a, b));
}
