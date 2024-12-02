#[derive(Eq, PartialEq)]
enum LevelState {
    Unknown,
    Decreasing,
    Increasing,
}

fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/inputs/2"));

    let part1 = part1(input);
    println!("part1 {}", part1);

    let part2 = part2(input);
    println!("part2 {}", part2);
}

fn part1(levels: &str) -> i64 {
    let mut part1 = 0;
    for l in levels.split("\n") {
        let level = l
            .split(" ")
            .map(|s| s.parse().unwrap())
            .collect::<Vec<i64>>();
        let safe = is_level_safe(&level);

        if safe {
            part1 += 1;
        }
    }
    part1
}

fn is_level_safe(level: &[i64]) -> bool {
    let mut state = LevelState::Unknown;
    let mut safe = true;
    for i in 0..level.len() - 1 {
        if i == 0 {
            if level[i] < level[i + 1] {
                state = LevelState::Increasing
            } else {
                state = LevelState::Decreasing
            }
        } else if (level[i] < level[i + 1] && state == LevelState::Decreasing)
            || (level[i] > level[i + 1] && state == LevelState::Increasing)
        {
            safe = false;
            break;
        }
        let diff = (level[i] - level[i + 1]).abs();

        if !(1..=3).contains(&diff) {
            safe = false;
            break;
        }
    }

    safe
}

fn part2(levels: &str) -> i64 {
    let mut part2 = 0;
    for l in levels.split("\n") {
        let level = l
            .split(" ")
            .map(|s| s.parse().unwrap())
            .collect::<Vec<i64>>();
        let safe = is_level_safe(&level);

        if safe {
            part2 += 1;
        } else {
            for i in 0..level.len() {
                let mut new = level.clone();
                new.remove(i);
                if is_level_safe(&new) {
                    part2 += 1;
                    break;
                }
            }
        }
    }
    part2
}
