use std::{
    fs::File,
    io::Write,
};

fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/inputs/14"));

    let max_x = 100;
    let max_y = 102;
    // let max_x = 10;
    // let max_y = 6;
    let mut robots: Vec<Robot> = input.lines().map(Robot::from_str).collect();
    let initial_state = robots.clone();

    for _ in 0..100 {
        for r in &mut robots {
            r.sim_next(max_x, max_y);
        }
    }

    // Quadrants
    // Test
    // let (x0, x1, x2, x3) = (0, 4, 6, 10);
    // let (y0, y1, y2, y3) = (0, 2, 4, 6);

    let (x0, x1, x2, x3) = (0, 49, 51, 100);
    let (y0, y1, y2, y3) = (0, 50, 52, 102);

    let (q1, q2, q3, q4) = get_quadrant_counts(&robots, x0, x1, x2, x3, y0, y1, y2, y3);
    let part1 = q1 * q2 * q3 * q4;
    println!("part1: {}", part1);

    // wtf does a christmas tree look like with 500 robots?
    // part2:
    // Once have the cycle stop otherwise just write the grid for every possible one, and ctrl+f which one looks
    // like a christmas tree.
    let mut output = File::create("output2.txt").unwrap();
    let mut robots2 = initial_state.clone();
    let mut iter = 1;
    loop {
        for r in &mut robots2 {
            r.sim_next(max_x, max_y);
        }
        writeln!(output, "iter: {}", iter).unwrap();
        print_grid(max_x, max_y, &robots2, &output);

        let mut all_eq = true;
        for i in 0..robots2.len() {
            if initial_state[i].position != robots2[i].position {
                all_eq = false;
                break;
            }
        }

        if all_eq {
            break;
        }
        iter +=1;
    }
}

#[allow(clippy::too_many_arguments)]
fn get_quadrant_counts(
    robots: &[Robot],
    x0: i64,
    x1: i64,
    x2: i64,
    x3: i64,
    y0: i64,
    y1: i64,
    y2: i64,
    y3: i64,
) -> (i64, i64, i64, i64) {
    let q1 = robots.iter().fold(0, |acc, r| {
        if r.in_quadrant(x0, x1, y0, y1) {
            acc + 1
        } else {
            acc
        }
    });

    let q2 = robots.iter().fold(0, |acc, r| {
        if r.in_quadrant(x2, x3, y0, y1) {
            acc + 1
        } else {
            acc
        }
    });

    let q3 = robots.iter().fold(0, |acc, r| {
        if r.in_quadrant(x0, x1, y2, y3) {
            acc + 1
        } else {
            acc
        }
    });

    let q4 = robots.iter().fold(0, |acc, r| {
        if r.in_quadrant(x2, x3, y2, y3) {
            acc + 1
        } else {
            acc
        }
    });

    (q1, q2, q3, q4)
}

#[allow(unused)]
fn print_grid(max_x: i64, max_y: i64, robots: &[Robot], mut writer: impl std::io::Write) {
    for j in 0..=max_y {
        for i in 0..=max_x {
            let mut count = 0;
            for r in robots {
                if r.position == (i, j) {
                    count += 1;
                }
            }
            if count > 0 {
                write!(writer, "{}", count).unwrap();
            } else {
                write!(writer, ".").unwrap();
            }
        }
        writeln!(writer).unwrap();
    }
}

#[derive(Clone)]
struct Robot {
    position: (i64, i64),
    velocity: (i64, i64),
}

impl Robot {
    fn from_str(s: &str) -> Self {
        let (p, v) = s.strip_prefix("p=").unwrap().split_once(" v=").unwrap();
        let p = p.split_once(",").unwrap();
        let v = v.split_once(',').unwrap();

        Self {
            position: (p.0.parse().unwrap(), p.1.parse().unwrap()),
            velocity: (v.0.parse().unwrap(), v.1.parse().unwrap()),
        }
    }

    fn sim_next(&mut self, max_x: i64, max_y: i64) {
        let mut next_x = self.position.0 + self.velocity.0;
        let mut next_y = self.position.1 + self.velocity.1;

        if next_x > max_x {
            next_x = next_x - max_x - 1;
        }

        if next_x < 0 {
            next_x = max_x + next_x + 1;
        }

        if next_y > max_y {
            next_y = next_y - max_y - 1;
        }

        if next_y < 0 {
            next_y = max_y + next_y + 1;
        }

        self.position = (next_x, next_y);
    }

    fn in_quadrant(&self, x_lower: i64, x_higher: i64, y_lower: i64, y_higher: i64) -> bool {
        self.position.0 <= x_higher
            && self.position.0 >= x_lower
            && self.position.1 >= y_lower
            && self.position.1 <= y_higher
    }
}
