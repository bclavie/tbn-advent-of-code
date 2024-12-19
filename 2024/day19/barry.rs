use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/inputs/19"));

    let problems = Problem::from_input(input);

    let part1 = problems.solve();
    println!("part1: {}", part1);

    let part2 = problems.solve2();
    println!("part2: {}", part2);
}

#[derive(Debug)]
struct Problem {
    towels: HashSet<String>,
    problems: Vec<String>,
}

impl Problem {
    fn from_input(input: &str) -> Self {
        let (towels, designs) = input.split_once("\n\n").unwrap();
        let towels = towels.split(',').map(|s| s.trim().to_string()).collect();
        let problems: Vec<String> = designs.split('\n').map(|s| s.to_string()).collect();

        Self {
            towels,
            problems,
        }
    }

    fn solve(&self) -> i64 {
        let mut available = 0;

        for p in &self.problems {
            if self.is_possible(p.clone()) {
                available += 1;
            }
        }

        available
    }

    fn is_possible(&self, problem: String) -> bool {
        if problem.is_empty() {
            return true;
        }
        for t in &self.towels {
            if let Some(trimmed) = problem.strip_prefix(t) {
                if self.is_possible(trimmed.to_string()) {
                    return true;
                }
            }
        }

        false
    }

    fn solve2(&self) -> i64 {
        let mut possible_combos = 0;

        // For a given substring - how many ways can we make it?
        let mut seen: HashMap<String, i64> = HashMap::<String, i64>::new();
        for p in &self.problems {
            possible_combos += self.possible_combos(&mut seen, p.clone());
        }

        possible_combos
    }

    fn possible_combos(&self, cache: &mut HashMap<String, i64>, problem: String) -> i64 {
        if let Some(num) = cache.get(&problem) {
            return *num;
        }
        if problem.is_empty() {
            return 1;
        }

        let mut total = 0;
        for t in &self.towels {
            if let Some(trimmed) = problem.strip_prefix(t) {
                let possible = self.possible_combos(cache, trimmed.to_string());
                cache.insert(trimmed.to_string(), possible);
                total += possible;
            }
        }
        total
    }
}
