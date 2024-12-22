use std::collections::{HashMap, VecDeque};

fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/inputs/20"));

    let grid = Grid::from_input(input);

    let path: HashMap<(usize, usize), i64> = grid.find_best_path();
    let part1 = grid.count_cheats(path.clone(), 100);
    println!("part1: {}", part1);

    // part2:
    // can we just use the distances from the path map?
    // compare hamming distance between two points if the hamming distance is < actual distance
    // we can cheat to get there?
    // 890554 too low
    let part2 = grid.count_cheats2(path, 100, 20);
    println!("part2: {}", part2);
}

fn ham_dist(l: (usize, usize), r: (usize, usize)) -> i64 {
    let x_diff = (l.0 as i64 - r.0 as i64).abs();
    let y_diff = (l.1 as i64 - r.1 as i64).abs();

    x_diff + y_diff
}

struct Grid {
    start: (usize, usize),
    end: (usize, usize),
    grid: Vec<Vec<char>>,
    x_max: usize,
    y_max: usize,
}

impl Grid {
    fn from_input(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let mut start: (usize, usize) = (0, 0);
        let mut end: (usize, usize) = (0, 0);
        for j in 0..grid.len() {
            for i in 0..grid[1].len() {
                if grid[j][i] == 'S' {
                    start = (i, j);
                }
                if grid[j][i] == 'E' {
                    end = (i, j)
                }
            }
        }
        let x_max = grid[0].len() - 1;
        let y_max = grid.len() - 1;
        Self {
            start,
            end,
            grid,
            x_max,
            y_max,
        }
    }

    // Get each coord on the path, and the distance from start.
    fn find_best_path(&self) -> HashMap<(usize, usize), i64> {
        // Standard bfs/
        let mut path = HashMap::new();

        let mut queue = VecDeque::new();
        path.insert(self.start, 0);
        queue.push_front((self.start, 0));

        while let Some(node) = queue.pop_front() {
            path.insert(node.0, node.1);
            if node.0 == self.end {
                break;
            }

            let next_distance = node.1 + 1;
            if node.0 .0 > 0 {
                let next = (node.0 .0 - 1, node.0 .1);
                if !path.contains_key(&next) && self.grid[next.1][next.0] != '#' {
                    queue.push_back((next, next_distance));
                }
            }

            if node.0 .0 < self.x_max {
                let next = (node.0 .0 + 1, node.0 .1);
                if !path.contains_key(&next) && self.grid[next.1][next.0] != '#' {
                    queue.push_back((next, next_distance));
                }
            }

            if node.0 .1 > 0 {
                let next = (node.0 .0, node.0 .1 - 1);
                if !path.contains_key(&next) && self.grid[next.1][next.0] != '#' {
                    queue.push_back((next, next_distance));
                }
            }

            if node.0 .1 < self.y_max {
                let next = (node.0 .0, node.0 .1 + 1);
                if !path.contains_key(&next) && self.grid[next.1][next.0] != '#' {
                    queue.push_back((next, next_distance));
                }
            }
        }

        path
    }

    fn count_cheats(&self, path: HashMap<(usize, usize), i64>, cheat_threshold: i64) -> i64 {
        let mut count = 0;
        let mut cheats = Vec::<((usize, usize), (usize, usize), i64)>::new();
        for (node, distance) in path.iter() {
            {
                let wall = (node.0 - 1, node.1);
                if wall.0 > 0 && self.grid[wall.1][wall.0] == '#' {
                    let left = (wall.0 - 1, wall.1);
                    if let Some(other) = path.get(&left) {
                        // Have to take 2 becuase we're spending 2 seconds cheating.
                        let distance = other - distance - 2;
                        if distance >= cheat_threshold {
                            cheats.push((*node, left, distance));
                            count += 1;
                        }
                    }
                }
            }

            {
                let wall = (node.0 + 1, node.1);
                if wall.0 < self.x_max && self.grid[wall.1][wall.0] == '#' {
                    let right = (wall.0 + 1, wall.1);
                    if let Some(other) = path.get(&right) {
                        let distance = other - distance - 2;

                        if distance >= cheat_threshold {
                            cheats.push((*node, right, distance));
                            count += 1;
                        }
                    }
                }
            }

            {
                let wall = (node.0, node.1 - 1);
                if wall.1 > 0 && self.grid[wall.1][wall.0] == '#' {
                    let down = (wall.0, wall.1 - 1);
                    if let Some(other) = path.get(&down) {
                        let distance = other - distance - 2;

                        if distance >= cheat_threshold {
                            cheats.push((*node, down, distance));
                            count += 1;
                        }
                    }
                }
            }

            {
                let wall = (node.0, node.1 + 1);
                if wall.1 < self.y_max && self.grid[wall.1][wall.0] == '#' {
                    let up = (wall.0, wall.1 + 1);
                    if let Some(other) = path.get(&up) {
                        let distance = other - distance - 2;

                        if distance >= cheat_threshold {
                            cheats.push((*node, up, distance));
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }

    fn count_cheats2(
        &self,
        path: HashMap<(usize, usize), i64>,
        cheat_threshold: i64,
        max_time: i64,
    ) -> i64 {
        let mut count = 0;
        let mut path_by_dist: Vec<((usize, usize), i64)> =
            path.into_iter().map(|item| (item.0, item.1)).collect();
        path_by_dist.sort_by(|a, b| a.1.cmp(&b.1));

        for i in 0..path_by_dist.len() {
            for j in i + 1..path_by_dist.len() {
                let start = path_by_dist[i];
                let end = path_by_dist[j];
                let path_distance: i64 = end.1 - start.1;
                if path_distance < cheat_threshold {
                    continue;
                }

                let cheat_time = ham_dist(start.0, end.0);
                let time_saved = path_distance - cheat_time;
                if cheat_time <= max_time && time_saved >= cheat_threshold {
                    count += 1;
                }
            }
        }

        count
    }
}
