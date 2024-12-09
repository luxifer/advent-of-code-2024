use advent_of_code::utils;

fn main() {
    let input = std::env::args().nth(1).expect("missing input");

    let mut disk_map: Vec<u32> = Vec::new();

    if let Ok(lines) = utils::read_lines(input) {
        for line in lines.flatten() {
            for c in line.chars() {
                disk_map.push(c.to_digit(10).unwrap());
            }
        }
    }

    let checksum = defrag_disk(&disk_map);

    println!("answer: {}", checksum);

    let checksum = compact_disk(&disk_map);

    println!("answer: {}", checksum);
}

fn compact_disk(disk_map: &Vec<u32>) -> i64 {
    let mut disk_blocks = map_to_block(&disk_map);
    let last_id = find_last_id(&disk_blocks);

    for id in (0..=last_id).rev() {
        // println!("## {}", id);

        let (i, lenght, ok) = find_block(&disk_blocks, id);

        if !ok {
            return 0;
        }

        let (j, ok) = find_empty_space(&disk_blocks, lenght, i);

        if !ok {
            // println!("no free space");
            continue;
        }

        (0..lenght).for_each(|p| disk_blocks.swap(i + p as usize, j + p as usize));
    }

    return checksum(&disk_blocks);
}

fn find_empty_space(disk_blocks: &Vec<i32>, size: i32, boundary: usize) -> (usize, bool) {
    let mut length: usize = 0;

    for i in 0..boundary {
        if disk_blocks[i] >= 0 {
            length = 0;
            continue;
        }
        length += 1;

        if length >= size as usize {
            return (i - length + 1, true);
        }
    }

    return (0, false);
}

fn find_block(disk_blocks: &Vec<i32>, id: i32) -> (usize, i32, bool) {
    let mut last_pos: usize = 0;

    for i in (0..disk_blocks.len()).rev() {
        if disk_blocks[i] > id || disk_blocks[i] == -1 {
            continue;
        }
        last_pos = i;
        break;
    }

    let mut length = 0;
    while disk_blocks[last_pos] == id {
        if last_pos == 0 {
            break;
        }

        length += 1;
        last_pos -= 1;
    }

    if length > 0 {
        return (last_pos + 1, length, true);
    }

    return (0, 0, false);
}

fn find_last_id(disk_blocks: &Vec<i32>) -> i32 {
    for i in (0..disk_blocks.len()).rev() {
        if disk_blocks[i] > 0 {
            return disk_blocks[i];
        }
    }

    return -1;
}

fn defrag_disk(disk_map: &Vec<u32>) -> i64 {
    let mut disk_blocks = map_to_block(&disk_map);
    let mut last_pos: usize = disk_blocks.len() - 1;
    let empty = disk_blocks.iter().filter(|&p| *p == -1).count();

    for i in 0..disk_blocks.len() - empty {
        if disk_blocks[i] == -1 {
            disk_blocks.swap(i, last_pos);

            while disk_blocks[last_pos] == -1 {
                last_pos -= 1;
            }
        }
    }

    return checksum(&disk_blocks);
}

fn checksum(disk_blocks: &Vec<i32>) -> i64 {
    let mut total: i64 = 0;

    for i in 0..disk_blocks.len() {
        if disk_blocks[i] == -1 {
            continue;
        }
        total += disk_blocks[i] as i64 * i as i64;
    }

    return total;
}

fn map_to_block(disk_map: &Vec<u32>) -> Vec<i32> {
    let mut file_idx = 0;
    let mut disk_blocks: Vec<i32> = Vec::new();

    for i in 0..disk_map.len() {
        let block_count = disk_map[i];
        if i % 2 == 0 {
            (0..block_count).for_each(|_| disk_blocks.push(file_idx));
            file_idx += 1;
        } else {
            (0..block_count).for_each(|_| disk_blocks.push(-1));
        }
    }

    return disk_blocks;
}
