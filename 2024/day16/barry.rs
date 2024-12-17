use std::collections::{BinaryHeap, HashMap, HashSet};

fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/inputs/16"));
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut start_position: (usize, usize) = (0, 0);
    'outer: for j in 0..grid.len() {
        for i in 0..grid[0].len() {
            if grid[j][i] == 'S' {
                start_position = (i, j);
                break 'outer;
            }
        }
    }

    let (part1, part2) = search(start_position, &grid);
    println!("part1: {}", part1);
    println!("part2: {}", part2);
}

// Search for best time using A*.
fn search(start_position: (usize, usize), grid: &[Vec<char>]) -> (i64, i64) {
    let score = 0;
    let mut queue = BinaryHeap::new();

    let mut best_score = i64::MAX;
    let mut best_path_coords = HashSet::<(usize, usize)>::new();

    // Every positon + direction pair keeps track of the best score so we can optimise away paths.
    let mut best_scores: HashMap<((usize, usize), Direction), i64> = HashMap::new();
    let mut path = HashSet::new();
    path.insert(start_position);
    queue.push(State {
        position: start_position,
        score,
        direction: Direction::East,
        current_path: path,
    });

    while let Some(current) = queue.pop() {
        if let Some(value) = best_scores.get_mut(&(current.position, current.direction)) {
            if current.score <= *value {
                *value = current.score;
            } else {
                continue;
            }
        } else {
            best_scores.insert((current.position, current.direction), current.score);
        }
        if current.score > best_score {
            continue;
        }
        if grid[current.position.1][current.position.0] == 'E' {
            best_score = current.score;
            for p in &current.current_path {
                best_path_coords.insert(*p);
            }
        }

        {
            let new_direction = current.direction.turn_anti_clockwise();
            let new_score = current.score + 1000;
            queue.push(State {
                position: current.position,
                score: new_score,
                direction: new_direction,
                current_path: current.current_path.clone(),
            });
        }

        {
            let new_direction = current.direction.turn_clockwise();
            let new_score: i64 = current.score + 1000;
            queue.push(State {
                position: current.position,
                score: new_score,
                direction: new_direction,
                current_path: current.current_path.clone(),
            });
        }

        // Go forward
        {
            match current.direction {
                Direction::East => {
                    if grid[current.position.1][current.position.0 + 1] != '#' {
                        let new_position = (current.position.0 + 1, current.position.1);
                        let mut next_path = current.current_path.clone();
                        next_path.insert((current.position.0 + 1, current.position.1));
                        queue.push(State {
                            position: new_position,
                            score: current.score + 1,
                            direction: current.direction,
                            current_path: next_path,
                        });
                        continue;
                    }
                }
                Direction::North => {
                    if grid[current.position.1 - 1][current.position.0] != '#' {
                        let new_position = (current.position.0, current.position.1 - 1);
                        let mut next_path = current.current_path;
                        next_path.insert((current.position.0, current.position.1 - 1));
                        queue.push(State {
                            position: new_position,
                            score: current.score + 1,
                            direction: current.direction,
                            current_path: next_path,
                        });
                        continue;
                    }
                }
                Direction::West => {
                    if grid[current.position.1][current.position.0 - 1] != '#' {
                        let new_position = (current.position.0 - 1, current.position.1);
                        let mut next_path = current.current_path;
                        next_path.insert((current.position.0 - 1, current.position.1));
                        queue.push(State {
                            position: new_position,
                            score: current.score + 1,
                            direction: current.direction,
                            current_path: next_path,
                        });
                        continue;
                    }
                }
                Direction::South => {
                    if grid[current.position.1 + 1][current.position.0] != '#' {
                        let new_position = (current.position.0, current.position.1 + 1);
                        let mut next_path = current.current_path;
                        next_path.insert((current.position.0, current.position.1 + 1));
                        queue.push(State {
                            position: new_position,
                            score: current.score + 1,
                            direction: current.direction,
                            current_path: next_path,
                        });
                        continue;
                    }
                }
            }
        }
    }

    (best_score, best_path_coords.len() as i64)
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct State {
    position: (usize, usize),
    score: i64,
    direction: Direction,
    current_path: HashSet<(usize, usize)>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // flip so we are a min heap.
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq, Eq, Debug, Hash, Copy, Clone)]
enum Direction {
    East,
    North,
    West,
    South,
}

impl Direction {
    fn turn_anti_clockwise(&self) -> Self {
        match self {
            Direction::East => Direction::North,
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
        }
    }

    fn turn_clockwise(&self) -> Self {
        match self {
            Direction::East => Direction::South,
            Direction::North => Direction::East,
            Direction::West => Direction::North,
            Direction::South => Direction::West,
        }
    }
}
