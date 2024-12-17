use advent_of_code::cli;
use advent_of_code::matrix::coord;
use advent_of_code::matrix::matrix;
use advent_of_code::matrix::vector;
use anyhow::Result;
use regex::Regex;

fn main() -> Result<()> {
    let app = cli::Cli::new();
    let lines = app.content()?;

    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)")?;

    let mut robots: Vec<Robot> = Vec::new();

    for (_, [px, py, vx, vy]) in re.captures_iter(&lines).map(|c| c.extract()) {
        robots.push(Robot {
            pos: coord::Coord {
                x: px.parse::<i32>()?,
                y: py.parse::<i32>()?,
            },
            vel: vector::Vector {
                x: vx.parse::<i32>()?,
                y: vy.parse::<i32>()?,
            },
        });
    }

    cli::stage(1, || -> i64 {
        let mut outside: matrix::Matrix<i32> = matrix::Matrix::from_size(101, 103, 0);
        let mut robots_stage1 = robots.clone();

        for _ in 1..=100 {
            outside.reset(0);
            for robot in robots_stage1.iter_mut() {
                move_robot(robot, &outside);
                let count = outside.at_coord(robot.pos).unwrap();
                outside.set(robot.pos, count + 1);
            }
        }

        // println!("{}", outside);
        let middle_width = (outside.width - 1) / 2;
        let middle_height = (outside.height - 1) / 2;
        let (mut nw, mut ne, mut se, mut sw) = (0, 0, 0, 0);

        for c in outside.iter() {
            let count = outside.at_coord(c).unwrap();

            if c.x < middle_width && c.y < middle_height {
                nw += count;
            }
            if c.x > middle_width && c.y < middle_height {
                ne += count;
            }
            if c.x > middle_width && c.y > middle_height {
                se += count;
            }
            if c.x < middle_width && c.y > middle_height {
                sw += count;
            }
        }

        // println!("{}, {}, {}, {}", nw, ne, se, sw);

        return (nw * ne * se * sw) as i64;
    });

    cli::stage(2, || -> i64 {
        let mut outside: matrix::Matrix<i32> = matrix::Matrix::from_size(101, 103, 0);
        let mut robots_stage2 = robots.clone();
        let mut total = 0;

        loop {
            total += 1;
            outside.reset(0);
            for robot in robots_stage2.iter_mut() {
                move_robot(robot, &outside);
                let count = outside.at_coord(robot.pos).unwrap();
                outside.set(robot.pos, count + 1);
            }

            if search_tree(&outside, &robots_stage2) {
                print_tree(&outside);
                break;
            }
        }

        return total;
    });

    Ok(())
}

#[derive(Clone)]
struct Robot {
    pos: coord::Coord,
    vel: vector::Vector,
}

fn search_tree(m: &matrix::Matrix<i32>, robots: &Vec<Robot>) -> bool {
    for r in robots.iter() {
        let count = m.at_coord(r.pos).unwrap();

        if *count > 1 {
            return false;
        }
    }

    return true;
}

fn print_tree(m: &matrix::Matrix<i32>) {
    for (i, v) in m.data.iter().enumerate() {
        if i as i32 % m.width == 0 {
            println!();
        }

        if *v > 0 {
            print!("{}", v);
        } else {
            print!(" ");
        }
    }
    println!();
}

fn move_robot(r: &mut Robot, m: &matrix::Matrix<i32>) {
    let mut new_pos = r.pos.add(r.vel);

    if !m.in_coord(new_pos) {
        if new_pos.x < 0 {
            new_pos.x += m.width;
        } else if new_pos.x >= m.width {
            new_pos.x -= m.width;
        }

        if new_pos.y < 0 {
            new_pos.y += m.height;
        } else if new_pos.y >= m.height {
            new_pos.y -= m.height;
        }
    }

    r.pos = new_pos;
}
