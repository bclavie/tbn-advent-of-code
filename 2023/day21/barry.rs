use std::collections::{HashSet, VecDeque};

fn main() {
    let input = include_str!("inputs/21");
    let graph: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut start = (0, 0);
    for (y, _) in graph.iter().enumerate() {
        for (x, _) in graph[y].iter().enumerate() {
            if graph[y][x] == 'S' {
                start = (y, x);
                break;
            }
        }
    }

    // Doing completeDFS/BFS graph traversal is too computationally complex.
    // Instead consider that each "step" is a fixed amount.
    // A step forward and back is two steps that result in the same place.
    // So it is all the locations that are traversable in the max distance,
    // and even distance from the beginning.
    let possible_locations = traverse_graph(&graph, 64, start);
    let part1 = find_possibilities(&possible_locations, start, 64);
    println!("part1: {}", part1);

    // Part 2 needs a geometric solution. No shot for me to get this without help:
    // https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21
    let all_possible_locations = traverse_graph(&graph, 131, start);
    let even_visited_locations = all_possible_locations
        .iter()
        .filter(|val| distance(**val, start) % 2 == 0)
        .count();
    let odd_visited_locations = all_possible_locations
        .iter()
        .filter(|val| distance(**val, start) % 2 == 1)
        .count();
    let even_locations_from_center = all_possible_locations
        .iter()
        .filter(|val| distance(**val, start) % 2 == 0 && distance(**val, start) > 65)
        .count();
    let odd_locations_from_center = all_possible_locations
        .iter()
        .filter(|val| distance(**val, start) % 2 == 1 && distance(**val, start) > 65)
        .count();

    println!(
        "total odd: {}, total even:{}, odd_corners: {}, even_corners: {}",
        odd_visited_locations,
        even_visited_locations,
        odd_locations_from_center,
        even_locations_from_center
    );

    let total_steps = 26501365;
    let n = (total_steps - (graph.len() / 2)) / graph.len();
    assert_eq!(n, 202300);

    // Brute force low values to make sure the answers are equal.
    // let possible_locations2 = traverse_graph_infinity(&graph, total_steps, start);
    // let part2_brute = find_possibilities2(&possible_locations2, (start.0 as i64, start.1 as i64), total_steps as i64);
    // println!("part2 brute: {}", part2_brute);

    let part2 = ((n + 1) * (n + 1)) * odd_visited_locations + (n * n) * even_visited_locations
        - (n + 1) * odd_locations_from_center
        + n * even_locations_from_center;
    println!("part2: {}", part2);
}

fn find_possibilities(
    possible_locations: &HashSet<(usize, usize)>,
    start: (usize, usize),
    max_distance: usize,
) -> u64 {
    let mut total_possible = 0;
    for loc in possible_locations {
        let dist = distance(*loc, start);
        if dist <= max_distance && (max_distance - dist) % 2 == 0 {
            total_possible += 1;
        }
    }
    total_possible
}

fn distance(a: (usize, usize), b: (usize, usize)) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

#[allow(unused)]
fn find_possibilities2(
    possible_locations: &HashSet<(i64, i64)>,
    start: (i64, i64),
    max_distance: i64,
) -> u64 {
    let mut total_possible = 0;
    for loc in possible_locations {
        let dist = distance2(*loc, start);
        if dist <= max_distance && (max_distance - dist) % 2 == 0 {
            total_possible += 1;
        }
    }
    total_possible
}

#[allow(unused)]
fn distance2(a: (i64, i64), b: (i64, i64)) -> i64 {
    (a.0.abs_diff(b.0) + a.1.abs_diff(b.1)).try_into().unwrap()
}

fn traverse_graph(
    graph: &[Vec<char>],
    total_steps: usize,
    start: (usize, usize),
) -> HashSet<(usize, usize)> {
    let mut visited_locations = HashSet::new();

    let mut queue = VecDeque::<(usize, (usize, usize))>::new();
    queue.push_front((0, start));

    while let Some((step_count, current_location)) = queue.pop_front() {
        if step_count <= total_steps {
            // up
            if current_location.0 > 0 {
                match graph[current_location.0 - 1][current_location.1] {
                    'S' | '.' => {
                        let new_location = (current_location.0 - 1, current_location.1);
                        if !visited_locations.contains(&new_location) {
                            visited_locations.insert(new_location);
                            queue.push_back((step_count + 1, new_location))
                        }
                    }
                    _ => {}
                }
            }
            // down
            if current_location.0 < graph.len() - 1 {
                match graph[current_location.0 + 1][current_location.1] {
                    'S' | '.' => {
                        let new_location = (current_location.0 + 1, current_location.1);
                        if !visited_locations.contains(&new_location) {
                            visited_locations.insert(new_location);
                            queue.push_back((step_count + 1, new_location))
                        }
                    }
                    _ => {}
                }
            }
            // left
            if current_location.1 > 0 {
                match graph[current_location.0][current_location.1 - 1] {
                    'S' | '.' => {
                        let new_location = (current_location.0, current_location.1 - 1);
                        if !visited_locations.contains(&new_location) {
                            visited_locations.insert(new_location);
                            queue.push_back((step_count + 1, new_location))
                        }
                    }
                    _ => {}
                }
            }
            // right
            if current_location.1 < graph[0].len() - 1 {
                match graph[current_location.0][current_location.1 + 1] {
                    'S' | '.' => {
                        let new_location = (current_location.0, current_location.1 + 1);
                        if !visited_locations.contains(&new_location) {
                            visited_locations.insert(new_location);
                            queue.push_back((step_count + 1, new_location))
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    visited_locations
}

#[allow(unused)]
fn map_i64_coord_to_usize(c: (i64, i64), y_bound: usize, x_bound: usize) -> (usize, usize) {
    (
        c.0.rem_euclid(y_bound as i64) as usize,
        (c.1.rem_euclid(x_bound as i64) as usize),
    )
}

#[allow(unused)]
fn traverse_graph_infinity(
    graph: &[Vec<char>],
    total_steps: usize,
    start: (usize, usize),
) -> HashSet<(i64, i64)> {
    let mut visited_locations = HashSet::new();

    let mut queue = VecDeque::<(usize, (i64, i64))>::new();
    let new_start = (start.0 as i64, start.1 as i64);
    queue.push_front((0, new_start));

    while let Some((step_count, current_location)) = queue.pop_front() {
        if step_count <= total_steps {
            let mut new_locations = [(0_i64, 0_i64); 4];
            new_locations[0] = (current_location.0 - 1, current_location.1);
            new_locations[1] = (current_location.0 + 1, current_location.1);
            new_locations[2] = (current_location.0, current_location.1 - 1);
            new_locations[3] = (current_location.0, current_location.1 + 1);

            for new_location in new_locations {
                let usize_location =
                    map_i64_coord_to_usize(new_location, graph.len(), graph[0].len());
                match graph[usize_location.0][usize_location.1] {
                    'S' | '.' => {
                        if !visited_locations.contains(&new_location) {
                            visited_locations.insert(new_location);
                            queue.push_back((step_count + 1, new_location))
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    visited_locations
}
