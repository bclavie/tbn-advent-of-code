use std::collections::HashSet;

fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/inputs/10"));
    let grid: Vec<Vec<i32>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap().try_into().unwrap_or(-1))
                .collect()
        })
        .collect();

    let mut zero_locations: Vec<(usize, usize)> = vec![];
    for j in 0..grid.len() {
        for i in 0..grid[0].len() {
            if grid[j][i] == 0 {
                zero_locations.push((i, j));
            }
        }
    }
    println!("{:?}", zero_locations);

    let mut scores: Vec<i32> = vec![];
    for start in zero_locations.clone() {
        let mut found_nines = HashSet::new();
        let mut visited = HashSet::new();
        if start.0 > 0 && grid[start.1][start.0 - 1] == 1 {
            traverse_part1(
                &grid,
                start.0 - 1,
                start.1,
                1,
                &mut found_nines,
                &mut visited,
            );
        }
        if start.1 > 0 && grid[start.1 - 1][start.0] == 1 {
            traverse_part1(
                &grid,
                start.0,
                start.1 - 1,
                1,
                &mut found_nines,
                &mut visited,
            );
        }
        if start.0 < grid[0].len() - 1 && grid[start.1][start.0 + 1] == 1 {
            traverse_part1(
                &grid,
                start.0 + 1,
                start.1,
                1,
                &mut found_nines,
                &mut visited,
            );
        }
        if start.1 < grid.len() - 1 && grid[start.1 + 1][start.0] == 1 {
            traverse_part1(
                &grid,
                start.0,
                start.1 + 1,
                1,
                &mut found_nines,
                &mut visited,
            );
        }

        scores.push(found_nines.len().try_into().unwrap());
    }

    println!("{:?}", scores);
    let part1: i32 = scores.iter().sum();
    println!("part1: {}", part1);

    let mut trailheads: Vec<i32> = vec![];
    for start in zero_locations {
        let mut total_paths = 0;
        if start.0 > 0 && grid[start.1][start.0 - 1] == 1 {
            traverse_part2(&grid, start.0 - 1, start.1, 1, &mut total_paths);
        }
        if start.1 > 0 && grid[start.1 - 1][start.0] == 1 {
            traverse_part2(&grid, start.0, start.1 - 1, 1, &mut total_paths);
        }
        if start.0 < grid[0].len() - 1 && grid[start.1][start.0 + 1] == 1 {
            traverse_part2(&grid, start.0 + 1, start.1, 1, &mut total_paths);
        }
        if start.1 < grid.len() - 1 && grid[start.1 + 1][start.0] == 1 {
            traverse_part2(&grid, start.0, start.1 + 1, 1, &mut total_paths);
        }

        trailheads.push(total_paths);
    }

    let part2: i32 = trailheads.iter().sum();
    println!("part2: {}", part2);
}

fn traverse_part1(
    grid: &[Vec<i32>],
    x: usize,
    y: usize,
    height: i32,
    found_nines: &mut HashSet<(usize, usize)>,
    visited: &mut HashSet<(usize, usize)>,
) {
    if visited.contains(&(x, y)) {
        return;
    }
    visited.insert((x, y));
    if height == 9 {
        found_nines.insert((x, y));
    }
    if x > 0 && grid[y][x - 1] == (height + 1) {
        traverse_part1(grid, x - 1, y, height + 1, found_nines, visited);
    }
    if y > 0 && grid[y - 1][x] == (height + 1) {
        traverse_part1(grid, x, y - 1, height + 1, found_nines, visited);
    }
    if x < grid[0].len() - 1 && grid[y][x + 1] == height + 1 {
        traverse_part1(grid, x + 1, y, height + 1, found_nines, visited);
    }
    if y < grid.len() - 1 && grid[y + 1][x] == height + 1 {
        traverse_part1(grid, x, y + 1, height + 1, found_nines, visited);
    }
}

fn traverse_part2(grid: &[Vec<i32>], x: usize, y: usize, height: i32, total_paths: &mut i32) {
    if height == 9 {
        *total_paths += 1;
    }

    if x > 0 && grid[y][x - 1] == (height + 1) {
        traverse_part2(grid, x - 1, y, height + 1, total_paths);
    }
    if y > 0 && grid[y - 1][x] == (height + 1) {
        traverse_part2(grid, x, y - 1, height + 1, total_paths);
    }
    if x < grid[0].len() - 1 && grid[y][x + 1] == height + 1 {
        traverse_part2(grid, x + 1, y, height + 1, total_paths);
    }
    if y < grid.len() - 1 && grid[y + 1][x] == height + 1 {
        traverse_part2(grid, x, y + 1, height + 1, total_paths);
    }
}
