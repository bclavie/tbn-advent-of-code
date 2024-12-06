use core::panic;
use std::collections::HashSet;

const WALL_OBSTACLE: char = '#';

fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/inputs/6"));

    let starting_grid: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();

    let max_x: i32 = (starting_grid[0].len() - 1).try_into().unwrap();
    let max_y: i32 = (starting_grid.len() - 1).try_into().unwrap();
    let mut start_position = (0_i32, 0_i32);
    let guard_char = '^';

    for y in 0..starting_grid.len() {
        for x in 0..starting_grid[0].len() {
            if starting_grid[y][x] == guard_char {
                start_position = (x.try_into().unwrap(), y.try_into().unwrap());
                break;
            }
        }
    }
    // Simulate Guard
    let mut obstacles = HashSet::<(i32, i32)>::new();
    for y in 0..starting_grid.len() {
        for x in 0..starting_grid[0].len() {
            if starting_grid[y][x] == WALL_OBSTACLE {
                obstacles.insert((x.try_into().unwrap(), y.try_into().unwrap()));
            }
        }
    }
    let path = find_path(start_position, guard_char, &obstacles, max_x, max_y).unwrap();
    let part1 = path.len();
    println!("part1: {}", part1);

    // For each place on the grid where the guard walked check if a loop can happen if we introduce an obstacle there.
    let mut proposed_obstacles = Vec::<(i32, i32)>::new();
    for coord in path {
        if coord == start_position {
            continue;
        }
        let mut obstacles_with_proposed = obstacles.clone();
        obstacles_with_proposed.insert(coord);
        if find_path(
            start_position,
            guard_char,
            &obstacles_with_proposed,
            max_x,
            max_y,
        )
        .is_none()
        {
            proposed_obstacles.push(coord);
        }
    }
    let part2 = proposed_obstacles.len();

    println!("part2: {}", part2);
}

fn find_path(
    mut guard_position: (i32, i32),
    mut guard_char: char,
    obstacles: &HashSet<(i32, i32)>,
    max_x: i32,
    max_y: i32,
) -> Option<HashSet<(i32, i32)>> {
    // The visited coords - used to get a list of obstacles for part 2.
    let mut path = HashSet::<(i32, i32)>::new();

    // Detect loops if we are in the same coord and direction.
    let mut cycle_detection_cache = HashSet::<(i32, i32, char)>::new();
    loop {
        if cycle_detection_cache.contains(&(guard_position.0, guard_position.1, guard_char)) {
            return None;
        }
        cycle_detection_cache.insert((guard_position.0, guard_position.1, guard_char));
        path.insert(guard_position);

        // Find next position and direction.
        let next_position = match guard_char {
            '^' => (guard_position.0, guard_position.1 - 1),
            '>' => (guard_position.0 + 1, guard_position.1),
            'v' => (guard_position.0, guard_position.1 + 1),
            '<' => (guard_position.0 - 1, guard_position.1),
            _ => panic!("unexpected"),
        };
        if obstacles.contains(&next_position) {
            match guard_char {
                '^' => {
                    guard_char = '>';
                }
                '>' => {
                    guard_char = 'v';
                }
                'v' => {
                    guard_char = '<';
                }
                '<' => {
                    guard_char = '^';
                }
                _ => panic!("unexpected"),
            }
            continue;
        }

        if !is_in_bounds(next_position, max_x, max_y) {
            break;
        }
        guard_position = next_position;
    }

    Some(path)
}

fn is_in_bounds(next_position: (i32, i32), max_x: i32, max_y: i32) -> bool {
    next_position.0 >= 0
        && next_position.0 <= max_x
        && next_position.1 >= 0
        && next_position.1 <= max_y
}

#[allow(unused)]
fn print_grid(grid: &[Vec<char>], path: &HashSet<(i32, i32)>) {
    println!();
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if path.contains(&(x.try_into().unwrap(), y.try_into().unwrap())) {
                print!("X");
            } else {
                print!("{}", grid[y][x])
            }
        }
        println!();
    }
}
