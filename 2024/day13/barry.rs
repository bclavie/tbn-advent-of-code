use std::collections::{BinaryHeap, HashSet};

fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/inputs/13"));

    let mut problem_vec = vec![];

    let problems = input.split("\n\n");
    for p in problems {
        let behaviour: Vec<&str> = p.splitn(3, '\n').collect();

        // A
        let (a_x, a_y) = behaviour[0]
            .strip_prefix("Button A: ")
            .unwrap()
            .split_once(", ")
            .unwrap();
        let a_x_chars: Vec<char> = a_x.chars().collect();
        let a_y_chars: Vec<char> = a_y.chars().collect();
        let a_x = a_x_chars[a_x_chars.len() - 2].to_digit(10).unwrap() * 10
            + a_x_chars[a_x_chars.len() - 1].to_digit(10).unwrap();
        let a_y = a_y_chars[a_y_chars.len() - 2].to_digit(10).unwrap() * 10
            + a_y_chars[a_y_chars.len() - 1].to_digit(10).unwrap();

        // B
        let (b_x, b_y) = behaviour[1]
            .strip_prefix("Button B: ")
            .unwrap()
            .split_once(", ")
            .unwrap();
        let b_x_chars: Vec<char> = b_x.chars().collect();
        let b_y_chars: Vec<char> = b_y.chars().collect();
        let b_x = b_x_chars[b_x_chars.len() - 2].to_digit(10).unwrap() * 10
            + b_x_chars[b_x_chars.len() - 1].to_digit(10).unwrap();
        let b_y = b_y_chars[b_y_chars.len() - 2].to_digit(10).unwrap() * 10
            + b_y_chars[b_y_chars.len() - 1].to_digit(10).unwrap();

        // Target
        let (target_x, target_y) = behaviour[2]
            .strip_prefix("Prize: ")
            .unwrap()
            .split_once(", ")
            .unwrap();

        let x: i64 = target_x.strip_prefix("X=").unwrap().parse().unwrap();
        let y: i64 = target_y.strip_prefix("Y=").unwrap().parse().unwrap();

        problem_vec.push(Problem::new(
            (a_x as i64, a_y as i64),
            (b_x as i64, b_y as i64),
            x,
            y,
        ));
    }

    let mut part1 = 0;
    for p in problem_vec.iter() {
        part1 += p.solve(false).unwrap_or_default()
    }

    println!("part1 {}", part1);

    let mut part2 = 0;
    for p in problem_vec {
        part2 += p.solve(true).unwrap_or_default()
    }

    println!("part2 {}", part2);
}

struct Problem {
    a_move: (i64, i64),
    b_move: (i64, i64),
    x_target: i64,
    y_target: i64,
}

impl Problem {
    fn new(a_move: (i64, i64), b_move: (i64, i64), x_target: i64, y_target: i64) -> Self {
        Self {
            a_move,
            b_move,
            x_target,
            y_target,
        }
    }

    // Just solve the simultaneous equations. I don't think there actually can be more than 1 solution.
    fn solve(&self, part2: bool) -> Option<i64> {
        let (x_target, y_target) = if part2 {
            (
                10000000000000 + self.x_target,
                10000000000000 + self.y_target,
            )
        } else {
            (self.x_target, self.y_target)
        };
        // Rearranging the equation on paper:
        let a = (self.b_move.0 * y_target - self.b_move.1 * x_target)
            / (self.b_move.0 * self.a_move.1 - self.a_move.0 * self.b_move.1);
        let b = (x_target - self.a_move.0 * a) / self.b_move.0;

        // check if we actually get the result due to integer divison funniness.
        if a >= 0
            && b >= 0
            && x_target == a * self.a_move.0 + b * self.b_move.0
            && y_target == a * self.a_move.1 + b * self.b_move.1
        {
            return Some(3 * a + b);
        }
        None
    }

    #[allow(unused)]
    // Doesn't work for part2.
    fn get_token_cost(&self) -> Option<i64> {
        let mut cache = HashSet::<Entry>::new();

        // priority queue based on current cost.
        let mut queue = BinaryHeap::new();
        // Current X Current Y, token cost.
        queue.push(Entry {
            current_x: 0,
            current_y: 0,
            cost: 0,
            a_presses: 0,
            b_presses: 0,
        });

        while let Some(entry) = queue.pop() {
            if cache.contains(&entry) {
                continue;
            }

            if entry.current_x == self.x_target && entry.current_y == self.y_target {
                return Some(entry.cost);
            }

            cache.insert(entry);

            if entry.a_presses > 100
                || entry.b_presses > 100
                || entry.current_x > self.x_target
                || entry.current_y > self.y_target
            {
                continue;
            }

            let next_a = Entry {
                current_x: entry.current_x + self.a_move.0,
                current_y: entry.current_y + self.a_move.1,
                cost: entry.cost + 3,
                a_presses: entry.a_presses + 1,
                b_presses: entry.b_presses,
            };
            queue.push(next_a);

            let next_b = Entry {
                current_x: entry.current_x + self.b_move.0,
                current_y: entry.current_y + self.b_move.1,
                cost: entry.cost + 1,
                a_presses: entry.a_presses,
                b_presses: entry.b_presses + 1,
            };
            queue.push(next_b);
        }

        None
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
struct Entry {
    current_x: i64,
    current_y: i64,
    cost: i64,
    a_presses: i64,
    b_presses: i64,
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // flip so we are a min heap.
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.cmp(self))
    }
}
