use regex::Regex;

const MAX_RED: i64 = 12;
const MAX_GREEN: i64 = 13;
const MAX_BLUE: i64 = 14;

fn main() {
    // part 1
    let inputs = include_str!("inputs/02");

    let re_green = Regex::new(r"(?<green>\d*) green").unwrap();
    let re_blue = Regex::new(r"(?<blue>\d*) blue").unwrap();
    let re_red = Regex::new(r"(?<red>\d*) red").unwrap();

    let games: Vec<Vec<Pull>> = inputs
        .split('\n')
        .map(|line| line.split_once(':').unwrap())
        .map(|(_, pulls)| {
            let mut parsed_pulls = vec![];
            for pull in pulls.split(';') {
                let parsed_greens = match re_green.captures(pull) {
                    Some(cap) => cap.name("green").unwrap().as_str().parse::<i64>().unwrap(),
                    None => 0,
                };
                let parsed_blues = match re_blue.captures(pull) {
                    Some(cap) => cap.name("blue").unwrap().as_str().parse::<i64>().unwrap(),
                    None => 0,
                };
                let parsed_reds = match re_red.captures(pull) {
                    Some(cap) => cap.name("red").unwrap().as_str().parse::<i64>().unwrap(),
                    None => 0,
                };
                parsed_pulls.push(Pull {
                    red: parsed_reds,
                    green: parsed_greens,
                    blue: parsed_blues,
                });
            }
            parsed_pulls
        })
        .collect();

    // part 1
    let mut sum = 0;
    for (i, pulls) in games.iter().enumerate() {
        let is_game_impossible = pulls.iter().map(|p| p.is_possible()).any(|x| !x);
        if !is_game_impossible {
            sum += i + 1;
        }
    }
    println!("{}", sum);

    // part 2
    let mut power_sum = 0;
    for pulls in games.iter() {
        let (min_red, min_green, min_blue) = pulls.iter().fold((0, 0, 0), |mut acc, p| {
            if p.red > acc.0 {
                acc.0 = p.red
            }
            if p.green > acc.1 {
                acc.1 = p.green
            }
            if p.blue > acc.2 {
                acc.2 = p.blue
            }
            acc
        });
        power_sum += min_red * min_blue * min_green;
    }

    println!("{}", power_sum);
}

struct Pull {
    red: i64,
    green: i64,
    blue: i64,
}

impl Pull {
    fn is_possible(&self) -> bool {
        self.red <= MAX_RED && self.green <= MAX_GREEN && self.blue <= MAX_BLUE
    }
}
