use std::collections::{HashSet, VecDeque};

fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/inputs/18"));

    let all_obstacles: Vec<(i64, i64)> = input
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();

    let mut initial_obstacles = HashSet::<(i64, i64)>::new();
    let obstacle_limit = 1024;
    for obstacle in all_obstacles.iter().take(obstacle_limit) {
        initial_obstacles.insert(*obstacle);
    }

    let part1 = bfs(&initial_obstacles, 70);
    println!("part1:{}", part1);

    let part2: (i64, i64);
    let mut current_obstacle = 1024;
    loop {
        initial_obstacles.insert(all_obstacles[current_obstacle]);
        if bfs(&initial_obstacles, 70) == -1 {
            part2 = all_obstacles[current_obstacle];
            break;
        }
        current_obstacle += 1;
    }
    println!("part2:{:?}", part2);
}

fn bfs(initial_obstacles: &HashSet<(i64, i64)>, target: i64) -> i64 {
    let mut shortest = -1;
    let end = (target, target);
    let mut visited = HashSet::<(i64, i64)>::new();
    // coord, length
    let mut queue = VecDeque::<(i64, i64, i64)>::new();
    queue.push_back((0, 0, 0));

    while let Some(current) = queue.pop_front() {
        let current_coord = (current.0, current.1);
        if visited.contains(&current_coord) {
            continue;
        }
        visited.insert(current_coord);
        if current_coord == end {
            shortest = current.2;
            break;
        }

        let new_distance = current.2 + 1;
        {
            let up = (current_coord.0, current_coord.1 - 1);
            if in_bounds(up, target) && !initial_obstacles.contains(&up) {
                queue.push_back((up.0, up.1, new_distance));
            }
        }

        {
            let down: (i64, i64) = (current_coord.0, current_coord.1 + 1);
            if in_bounds(down, target) && !initial_obstacles.contains(&down) {
                queue.push_back((down.0, down.1, new_distance));
            }
        }

        {
            let left: (i64, i64) = (current_coord.0 - 1, current_coord.1);
            if in_bounds(left, target) && !initial_obstacles.contains(&left) {
                queue.push_back((left.0, left.1, new_distance));
            }
        }

        {
            let right: (i64, i64) = (current_coord.0 + 1, current_coord.1);
            if in_bounds(right, target) && !initial_obstacles.contains(&right) {
                queue.push_back((right.0, right.1, new_distance));
            }
        }
    }

    shortest
}

fn in_bounds(coord: (i64, i64), max: i64) -> bool {
    coord.0 >= 0 && coord.0 <= max && coord.1 >= 0 && coord.1 <= max
}
