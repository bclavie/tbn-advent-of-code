use std::collections::HashMap;

fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/inputs/11"));
    let mut stones: Vec<String> = input.split(' ').map(|s| s.to_string()).collect();
    let stones2 = stones.clone();

    // part1: naive loop.
    let iterations = 25;
    for _ in 0..iterations {
        let mut new_stones: Vec<String> = vec![];
        for stone in &stones {
            if *stone == "0" {
                new_stones.push("1".to_string());
            } else if stone.len() % 2 == 0 {
                let (l, mut r) = stone.split_at(stone.len() / 2);
                new_stones.push(l.to_string());

                while let Some(k) = r.strip_prefix("0") {
                    r = k;
                }
                if !r.is_empty() {
                    new_stones.push(r.to_string());
                } else {
                    new_stones.push("0".to_string());
                }
            } else {
                let num: i64 = stone.parse().unwrap();
                let new_stone = (num * 2024).to_string();
                new_stones.push(new_stone);
            }
        }

        stones = new_stones;
    }

    let part1 = stones.len();
    println!("part1: {}", part1);

    // Stone Value, Iterations -> result
    let mut cache = HashMap::<(String, usize), usize>::new();
    // The original stone value, the iteration from that stone and finally the stone result.
    // let mut graph = StoneGraph::new();
    let max_iterations = 75;

    // We work downward (from max iterations), storing the result of all stones in the cache.
    let mut part2 = 0;
    for s in stones2 {
        part2 += workout_stones(s, max_iterations, &mut cache)
    }
    println!("part2: {}", part2);
}

fn workout_stones(
    value: String,
    iteration: usize,
    cache: &mut HashMap<(String, usize), usize>,
) -> usize {
    if iteration == 0 {
        return 1;
    }

    if let Some(result) = cache.get(&(value.clone(), iteration)) {
        return *result;
    }
    // Otherwise workout what the stone is and recurse with cache.
    let mut num_stones = 0;
    if value == "0" {
        num_stones = workout_stones("1".to_string(), iteration - 1, cache);
    } else if value.len() % 2 == 0 {
        let (l, mut r) = value.split_at(value.len() / 2);
        num_stones += workout_stones(l.to_string(), iteration - 1, cache);

        while let Some(k) = r.strip_prefix("0") {
            r = k;
        }
        if !r.is_empty() {
            num_stones += workout_stones(r.to_string(), iteration - 1, cache);
        } else {
            num_stones += workout_stones("0".to_string(), iteration - 1, cache);
        }
    } else {
        let num: i64 = value.parse().unwrap();
        let new_stone = (num * 2024).to_string();

        num_stones = workout_stones(new_stone.to_string(), iteration - 1, cache);
    }

    cache.insert((value, iteration), num_stones);

    num_stones
}
