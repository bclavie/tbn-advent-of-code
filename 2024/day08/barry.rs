use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/inputs/8"));

    let char_grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let max_x = (char_grid[0].len() - 1).try_into().unwrap();
    let max_y = (char_grid.len() - 1).try_into().unwrap();

    // Map of node type e.g. 'A' to coordinates.
    let mut node_locations = HashMap::<char, Vec<(i32, i32)>>::new();
    for (j, line) in char_grid.iter().enumerate() {
        for i in 0..line.len() {
            let node = char_grid[j][i];
            if node != '.' {
                node_locations
                    .entry(node)
                    .or_default()
                    .push((i.try_into().unwrap(), j.try_into().unwrap()));
            }
        }
    }

    // Part 1:
    // For each pair of nodes per node type, do some maths to find the antinodes.
    let mut unique_antinode_locations = HashSet::<(i32, i32)>::new();
    for (_, positions) in node_locations.clone() {
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let x_diff = positions[i].0 - positions[j].0;
                let y_diff = positions[i].1 - positions[j].1;

                // Add the diff to the first coord to get the antinode on one side.
                // Subtract from the second coord to get antinode on other side.
                let left_antinode = (positions[i].0 + x_diff, positions[i].1 + y_diff);
                if is_in_bounds(left_antinode, max_x, max_y) {
                    unique_antinode_locations.insert(left_antinode);
                }

                let right_antinode = (positions[j].0 - x_diff, positions[j].1 - y_diff);
                if is_in_bounds(right_antinode, max_x, max_y) {
                    unique_antinode_locations.insert(right_antinode);
                }
            }
        }
    }

    let part1 = unique_antinode_locations.len();
    println!("part1: {}", part1);

    // Part 2:
    let mut unique_antinode_locations2: HashSet<(i32, i32)> = HashSet::<(i32, i32)>::new();
    for (_, positions) in node_locations {
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let x_diff = positions[i].0 - positions[j].0;
                let y_diff = positions[i].1 - positions[j].1;

                // Same as part1:
                // Add the diff to the first coord to get the antinode on one side.
                // Subtract from the second coord to get antinode on other side.
                // but keep doing this until we are out of bounds.
                let mut left_antinode = (positions[i].0, positions[i].1);
                while is_in_bounds(left_antinode, max_x, max_y) {
                    unique_antinode_locations2.insert(left_antinode);
                    left_antinode = (left_antinode.0 + x_diff, left_antinode.1 + y_diff);
                }

                let mut right_antinode = (positions[j].0, positions[j].1);
                while is_in_bounds(right_antinode, max_x, max_y) {
                    unique_antinode_locations2.insert(right_antinode);
                    right_antinode = (right_antinode.0 - x_diff, right_antinode.1 - y_diff);
                }
            }
        }
    }
    let part2 = unique_antinode_locations2.len();
    println!("part2: {}", part2);
}

fn is_in_bounds(position: (i32, i32), max_x: i32, max_y: i32) -> bool {
    position.0 >= 0 && position.0 <= max_x && position.1 >= 0 && position.1 <= max_y
}
