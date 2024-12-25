fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/inputs/25"));
    let mut keys = vec![];
    let mut locks = vec![];

    for grid in input.split("\n\n") {
        let lines: Vec<&str> = grid.lines().collect();
        if lines[0] == "#####" {
            let mut heights: [u8; 5] = [0, 0, 0, 0, 0];

            for (i, height) in heights.iter_mut().enumerate() {
                let mut current_height = 0;
                for j in (1..6).rev() {
                    if lines[j].chars().nth(i).unwrap() == '#' {
                        current_height = j;
                        break;
                    }
                }
                *height = current_height as u8;
            }

            locks.push(Lock { heights });
        } else {
            let mut heights: [u8; 5] = [0, 0, 0, 0, 0];

            for (i, height) in heights.iter_mut().enumerate() {
                let mut current_height = 0;
                for (j, _) in lines.iter().enumerate().take(6).skip(1) {
                    if lines[j].chars().nth(i).unwrap() == '#' {
                        current_height = 6 - j;
                        break;
                    }
                }
                *height = current_height as u8;
            }

            keys.push(Key { heights });
        }
    }

    let mut part1 = 0;
    for k in &keys {
        for l in &locks {
            if !overlap(k, l) {
                part1 += 1;
            }
        }
    }

    println!("part1: {}", part1);
}

fn overlap(key: &Key, lock: &Lock) -> bool {
    for i in 0..5 {
        if key.heights[i] + lock.heights[i] > 5 {
            return true;
        }
    }
    false
}

#[derive(Debug)]
struct Key {
    heights: [u8; 5],
}

#[derive(Debug)]
struct Lock {
    heights: [u8; 5],
}
