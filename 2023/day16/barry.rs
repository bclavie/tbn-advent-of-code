use std::collections::HashSet;

use itertools::Itertools;

fn main() {
    let inputs = include_str!("inputs/16");
    let grid: Vec<Vec<(char, u64)>> = inputs
        .lines()
        .map(|l| l.chars().zip([0].repeat(l.len())).collect())
        .collect_vec();

    let mut grid_clone_part1 = grid.clone();

    let mut visited = HashSet::<((usize, usize), Direction)>::new();
    light_traversal(
        (0, 0),
        Direction::Right,
        &mut grid_clone_part1,
        &mut visited,
    );

    let part1: u64 = energised_count(&grid_clone_part1);
    println!("part1: {}", part1);

    let mut highest = 0;
    for start_y in 0..grid.len() {
        {
            let mut grid_clone = grid.clone();
            let mut visited = HashSet::<((usize, usize), Direction)>::new();
            light_traversal(
                (start_y, 0),
                Direction::Right,
                &mut grid_clone,
                &mut visited,
            );
            let score = energised_count(&grid_clone);
            if score > highest {
                highest = score
            }
        }

        {
            let mut grid_clone = grid.clone();
            let mut visited = HashSet::<((usize, usize), Direction)>::new();
            light_traversal(
                (start_y, grid[0].len() - 1),
                Direction::Left,
                &mut grid_clone,
                &mut visited,
            );
            let score = energised_count(&grid_clone);
            if score > highest {
                highest = score
            }
        }
    }

    for start_x in 0..grid[0].len() {
        {
            let mut grid_clone = grid.clone();
            let mut visited = HashSet::<((usize, usize), Direction)>::new();
            light_traversal((0, start_x), Direction::Down, &mut grid_clone, &mut visited);

            let score = energised_count(&grid_clone);
            if score > highest {
                highest = score
            }
        }

        {
            let mut grid_clone = grid.clone();
            let mut visited = HashSet::<((usize, usize), Direction)>::new();
            light_traversal(
                (grid.len() - 1, start_x),
                Direction::Up,
                &mut grid_clone,
                &mut visited,
            );
            let score = energised_count(&grid_clone);
            if score > highest {
                highest = score
            }
        }
    }

    println!("part2: {}", highest);
}

#[allow(unused)]
fn debug_print2(grid: &[Vec<(char, u64)>]) {
    println!();
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            print!("{}", grid[y][x].0)
        }
        println!()
    }
}

#[allow(unused)]
fn debug_print(grid: &[Vec<(char, u64)>]) {
    println!();
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x].1 > 0 {
                print!("{}", grid[y][x].1);
            } else {
                print!(".")
            }
        }
        println!()
    }
}

fn energised_count(grid: &[Vec<(char, u64)>]) -> u64 {
    grid.iter()
        .flatten()
        .map(|pos| if pos.1 > 0 { 1 } else { 0 })
        .sum()
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn light_traversal(
    start: (usize, usize),
    direction: Direction,
    grid: &mut [Vec<(char, u64)>],
    visited: &mut HashSet<((usize, usize), Direction)>,
) {
    let mut next = start;
    let mut dir = direction;
    while next.0 < grid.len() && next.1 < grid[0].len() {
        // If a beam of light has already gone through a point with the same direction, then exit
        if visited.contains(&(next, dir)) {
            return;
        }
        // Energise the grid
        visited.insert((next, dir));
        grid[next.0][next.1].1 += 1;

        match (dir, grid[next.0][next.1].0) {
            (Direction::Up, '.') | (Direction::Up, '|') => {
                if next.0 > 0 {
                    next = (next.0 - 1, next.1);
                } else {
                    return;
                }
            }
            (Direction::Up, '/') => {
                next = (next.0, next.1 + 1);
                dir = Direction::Right;
            }
            (Direction::Up, '\\') => {
                if next.1 > 0 {
                    next = (next.0, next.1 - 1);
                    dir = Direction::Left;
                } else {
                    return;
                }
            }
            (Direction::Up, '-') => {
                if next.1 > 0 {
                    light_traversal((next.0, next.1 - 1), Direction::Left, grid, visited);
                }
                next = (next.0, next.1 + 1);
                dir = Direction::Right;
            }

            (Direction::Down, '.') | (Direction::Down, '|') => {
                next = (next.0 + 1, next.1);
            }
            (Direction::Down, '/') => {
                if next.1 > 0 {
                    next = (next.0, next.1 - 1);
                    dir = Direction::Left;
                } else {
                    return;
                }
            }
            (Direction::Down, '\\') => {
                next = (next.0, next.1 + 1);
                dir = Direction::Right;
            }
            (Direction::Down, '-') => {
                if next.1 > 0 {
                    light_traversal((next.0, next.1 - 1), Direction::Left, grid, visited);
                }
                next = (next.0, next.1 + 1);
                dir = Direction::Right;
            }

            (Direction::Right, '.') | (Direction::Right, '-') => {
                next = (next.0, next.1 + 1);
            }
            (Direction::Right, '/') => {
                if next.0 > 0 {
                    next = (next.0 - 1, next.1);
                    dir = Direction::Up;
                } else {
                    return;
                }
            }
            (Direction::Right, '\\') => {
                next = (next.0 + 1, next.1);
                dir = Direction::Down;
            }
            (Direction::Right, '|') => {
                if next.0 > 0 {
                    light_traversal((next.0 - 1, next.1), Direction::Up, grid, visited);
                }
                next = (next.0 + 1, next.1);
                dir = Direction::Down;
            }

            (Direction::Left, '.') | (Direction::Left, '-') => {
                if next.1 > 0 {
                    next = (next.0, next.1 - 1);
                } else {
                    return;
                }
            }
            (Direction::Left, '/') => {
                next = (next.0 + 1, next.1);
                dir = Direction::Down;
            }
            (Direction::Left, '\\') => {
                if next.0 > 0 {
                    next = (next.0 - 1, next.1);
                    dir = Direction::Up;
                } else {
                    return;
                }
            }
            (Direction::Left, '|') => {
                if next.0 > 0 {
                    light_traversal((next.0 - 1, next.1), Direction::Up, grid, visited);
                }
                next = (next.0 + 1, next.1);
                dir = Direction::Down;
            }
            (_, _) => panic!("unexpected char"),
        }
    }
}
