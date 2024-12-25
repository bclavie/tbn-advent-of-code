use std::collections::HashMap;

fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/inputs/22"));

    let mut part1 = 0;
    let mut prices = Vec::<Prices>::new();
    for line in input.lines() {
        let mut secret = line.parse().unwrap();
        let init = secret;
        let mut all_prices = vec![];
        for _ in 0..2000 {
            let price = (secret % 10) as i8;
            all_prices.push(price);
            secret = next(secret);
        }
        prices.push(Prices::new(init, all_prices));
        part1 += secret;
    }

    println!("part1 {}", part1);

    let mut sequences_to_best_price = HashMap::<(i8, i8, i8, i8), i64>::new();
    for price in prices.iter() {
        for s in price.sequence_to_first_price.keys() {
            if sequences_to_best_price.contains_key(s) {
                continue;
            }

            let mut total_price = 0_i64;
            for p in prices.iter() {
                let price = match p.sequence_to_first_price.get(s) {
                    Some(p) => *p,
                    None => 0,
                };

                total_price += price as i64;
            }

            sequences_to_best_price.insert(*s, total_price);
        }
    }

    let part2 = sequences_to_best_price
        .iter()
        .max_by(|a, b| a.1.cmp(b.1))
        .unwrap();

    println!("part2 {:?}", part2.1);
}

#[allow(unused)]
struct Prices {
    init: i64,
    prices: Vec<i8>,
    sequence_to_first_price: HashMap<(i8, i8, i8, i8), i8>,
}

impl Prices {
    fn new(init: i64, prices: Vec<i8>) -> Self {
        let mut sequence_to_first_price = HashMap::new();

        for i in 0..prices.len() - 4 {
            let diff0 = prices[i + 1] - prices[i];
            let diff1 = prices[i + 2] - prices[i + 1];
            let diff2 = prices[i + 3] - prices[i + 2];
            let diff3 = prices[i + 4] - prices[i + 3];

            let price: i8 = prices[i + 4];

            sequence_to_first_price.entry((diff0, diff1, diff2, diff3)).or_insert(price);
        }

        Self {
            init,
            prices,
            sequence_to_first_price,
        }
    }
}

fn next(secret: i64) -> i64 {
    let mut current_secret = secret;
    // Left shift 6.
    let mut result = current_secret * 64;
    current_secret = mix(result, current_secret);
    current_secret = prune(current_secret);

    // Right shift 5
    result = current_secret / 32;
    current_secret = mix(result, current_secret);
    current_secret = prune(current_secret);

    // Left shift 12
    result = current_secret * 2048;
    current_secret = mix(result, current_secret);
    current_secret = prune(current_secret);

    current_secret
}

fn mix(value: i64, secret: i64) -> i64 {
    value ^ secret
}

// This is 0b1000000000000000000000000, max 25.
fn prune(value: i64) -> i64 {
    value % 16777216
}
