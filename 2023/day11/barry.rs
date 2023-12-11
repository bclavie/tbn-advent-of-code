use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn main() {
    let inputs = include_str!("inputs/11");
    let base_galaxy: Vec<Vec<char>> = inputs.lines().map(|l| l.chars().collect()).collect();
    let mut base_galaxy_indices = Vec::<(usize, usize)>::new();
    let mut empty_cols = HashSet::<usize>::new();
    let mut empty_rows = HashSet::<usize>::new();

    for y in 0..base_galaxy.len() {
        empty_rows.insert(y);
    }
    for x in 0..base_galaxy[0].len() {
        empty_cols.insert(x);
    }

    for (y, line) in base_galaxy.iter().enumerate() {
        for (x, point) in line.iter().enumerate() {
            match point {
                '#' => {
                    base_galaxy_indices.push((y, x));
                    empty_rows.remove(&y);
                    empty_cols.remove(&x);
                }
                '.' => {}
                _ => panic!("unexpected space"),
            }
        }
    }
    // expand the base galaxy
    let extra_cols = empty_cols.len();
    let extra_rows = empty_rows.len();
    let mut expanded_galaxy =
        vec![vec!['.'; base_galaxy[0].len() + extra_cols]; base_galaxy.len() + extra_rows];
    let new_galaxy_positions: Vec<(usize, usize)> = base_galaxy_indices
        .iter()
        .map(|(y, x)| {
            let rows_smaller = empty_rows.iter().filter(|row_index| row_index < &y).count();
            let cols_smaller = empty_cols.iter().filter(|col_index| col_index < &x).count();
            (y + rows_smaller, x + cols_smaller)
        })
        .collect();
    for (y, x) in new_galaxy_positions.iter() {
        expanded_galaxy[*y][*x] = '#';
    }

    // print_galaxy(&expanded_galaxy);

    // This ends up counting each pair twice, so divide the result by 2.
    let diffs: Vec<u64> = new_galaxy_positions
        .iter()
        .cartesian_product(new_galaxy_positions.clone())
        .map(|(galaxy_1, galaxy2)| {
            (galaxy_1.0.abs_diff(galaxy2.0) + galaxy_1.1.abs_diff(galaxy2.1)) as u64
        })
        .collect();
    let part1: u64 = diffs.iter().sum::<u64>() / 2;

    println!("part1: {} ", part1);

    // Part 2 is just part 1 except with a bigger expansion factor.
    let expansion_factor: usize = 1000000;
    let new_galaxy_positions2: Vec<(usize, usize)> = base_galaxy_indices
        .iter()
        .map(|(y, x)| {
            let rows_smaller = empty_rows.iter().filter(|row_index| row_index < &y).count();
            let cols_smaller = empty_cols.iter().filter(|col_index| col_index < &x).count();
            (
                *y + rows_smaller * (expansion_factor - 1),
                *x + cols_smaller * (expansion_factor - 1),
            )
        })
        .collect();

    let diffs2: Vec<u64> = new_galaxy_positions2
        .iter()
        .cartesian_product(new_galaxy_positions2.clone())
        .map(|(galaxy_1, galaxy2)| {
            (galaxy_1.0.abs_diff(galaxy2.0) + galaxy_1.1.abs_diff(galaxy2.1)) as u64
        })
        .collect();
    let part2: u64 = diffs2.iter().sum::<u64>() / 2;

    println!("part2: {} ", part2);
}

#[allow(unused)]
fn print_galaxy(galaxy: &[Vec<char>]) {
    println!();
    for line in galaxy {
        for c in line {
            print!("{}", c);
        }
        println!();
    }
}
