use advent_of_code::utils;

fn main() {
    let input = std::env::args().nth(1).expect("missing input");

    let mut reports: Vec<Vec<i32>> = Vec::new();

    if let Ok(lines) = utils::read_lines(input) {
        for line in lines.flatten() {
            let parts: Vec<&str> = line.split(" ").collect();
            let levels: Vec<i32> = parts.iter().map(|p| p.parse::<i32>().unwrap()).collect();
            reports.push(levels);
        }
    }

    let mut safe: i32 = 0;

    for levels in reports.iter() {
        if is_safe(levels.clone()) {
            safe += 1;
        }
    }

    println!("answer: {}", safe);

    let mut safe: i32 = 0;

    for levels in reports.iter() {
        let mut any_safe = false;

        for i in 0..levels.len() {
            let mut fixed: Vec<i32> = levels.to_vec();
            fixed.remove(i);

            if is_safe(fixed) {
                any_safe = true;
                break;
            }
        }

        if any_safe {
            safe += 1;
        }
    }

    println!("answer: {}", safe);
}

fn is_safe(levels: Vec<i32>) -> bool {
    let mut prev: i32 = -1;
    let mut is_safe: bool = true;
    let mut dir = 0;

    for level in levels.iter() {
        let mut new_dir = 0;
        if prev != -1 {
            let diff = prev.abs_diff(*level);
            if diff > 3 {
                is_safe = false;
                break;
            }

            if prev == *level {
                is_safe = false;
                break;
            }

            if prev < *level {
                new_dir = 1;
            } else {
                new_dir = -1
            }
        }

        if dir != 0 {
            if dir != new_dir {
                is_safe = false;
                break;
            }
        }

        prev = *level;
        dir = new_dir;
    }

    return is_safe;
}
