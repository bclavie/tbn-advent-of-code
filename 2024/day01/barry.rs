use std::collections::HashMap;

fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/inputs/1"));

    let mut left = Vec::<i64>::new();
    let mut right = Vec::<i64>::new();

    for l in input.split("\n") {
        let ids: Vec<&str> = l.splitn(2, "   ").collect();

        left.push(ids[0].parse().unwrap());
        right.push(ids[1].parse().unwrap());
    }

    left.sort();
    right.sort();

    let mut part1 = 0;
    for i in 0..left.len() {
        part1 += (left[i] - right[i]).abs();
    }

    println!("part1: {}", part1);

    let mut right_occurances = HashMap::<i64, i64>::new();
    for id in right {
        right_occurances
            .entry(id)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }
    let mut part2 = 0;
    for id in left {
        part2 += *right_occurances.entry(id).or_default() * id
    }

    println!("part2: {}", part2);
}
