use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

fn main() {
    let inputs = include_str!("inputs/17");
    let grid: Vec<Vec<u32>> = inputs
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    // A* Priority queue == current heat.
    let part1 = search(&grid);
    println!("part1: {}", part1);

    let part2 = search_with_ultra_crucible(&grid);
    println!("part2: {}", part2);
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct SearchState {
    last_3_moves: [Direction; 3],
    current_coord: (usize, usize),
    path: Vec<(usize, usize)>,
    total_heat_loss: u32,
}

impl SearchState {
    fn push_direction(&mut self, direction: Direction) {
        self.last_3_moves[0] = self.last_3_moves[1];
        self.last_3_moves[1] = self.last_3_moves[2];
        self.last_3_moves[2] = direction;
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
enum Direction {
    U,
    D,
    L,
    R,
    Unspecified,
}

fn search(grid: &[Vec<u32>]) -> u32 {
    let end = (grid.len() - 1, grid[0].len() - 1);

    let mut priority_queue = VecDeque::<SearchState>::new();
    // Cache optimisation -> If we're at a coordinate with the same last 3 directions - we are very likely in a cycle
    // of some kind - so use this to break out of loops.
    let mut seen_coords_with_paths = HashSet::<((usize, usize), [Direction; 3])>::new();

    let initial_search = SearchState {
        last_3_moves: [Direction::Unspecified; 3],
        current_coord: (0, 0),
        path: vec![(0, 0)],
        total_heat_loss: 0,
    };
    priority_queue.push_front(initial_search);

    while let Some(current_state) = priority_queue.pop_front() {
        // println!("{:?}", current_state);
        if current_state.current_coord == end {
            return current_state.total_heat_loss;
        }
        if seen_coords_with_paths
            .contains(&(current_state.current_coord, current_state.last_3_moves))
        {
            continue;
        }

        // Put adjacent coords onto the queue
        // Handle directions
        // Get the current direction we are facing - we are only allowed to continue straight
        // or rotate.
        let mut next_directions = HashSet::<Direction>::new();
        match current_state.last_3_moves[2] {
            Direction::U => {
                next_directions.insert(Direction::U);
                next_directions.insert(Direction::L);
                next_directions.insert(Direction::R);
            }
            Direction::D => {
                next_directions.insert(Direction::D);
                next_directions.insert(Direction::L);
                next_directions.insert(Direction::R);
            }
            Direction::L => {
                next_directions.insert(Direction::U);
                next_directions.insert(Direction::D);
                next_directions.insert(Direction::L);
            }
            Direction::R => {
                next_directions.insert(Direction::U);
                next_directions.insert(Direction::D);
                next_directions.insert(Direction::R);
            }
            Direction::Unspecified => {
                // this is the starting position - so only down and right are valid
                next_directions.insert(Direction::D);
                next_directions.insert(Direction::R);
            }
        }
        if current_state
            .last_3_moves
            .iter()
            .all(|d| *d == Direction::D)
        {
            next_directions.remove(&Direction::D);
        }
        if current_state
            .last_3_moves
            .iter()
            .all(|d| *d == Direction::U)
        {
            next_directions.remove(&Direction::U);
        }
        if current_state
            .last_3_moves
            .iter()
            .all(|d| *d == Direction::L)
        {
            next_directions.remove(&Direction::L);
        }
        if current_state
            .last_3_moves
            .iter()
            .all(|d| *d == Direction::R)
        {
            next_directions.remove(&Direction::R);
        }

        // Handle the new directions:
        for d in next_directions {
            let mut next_state = current_state.clone();
            let new_coord = match d {
                Direction::U => {
                    if current_state.current_coord.0 == 0 {
                        continue;
                    } else {
                        (
                            current_state.current_coord.0 - 1,
                            current_state.current_coord.1,
                        )
                    }
                }
                Direction::D => {
                    if current_state.current_coord.0 == grid.len() - 1 {
                        continue;
                    } else {
                        (
                            current_state.current_coord.0 + 1,
                            current_state.current_coord.1,
                        )
                    }
                }
                Direction::L => {
                    if current_state.current_coord.1 == 0 {
                        continue;
                    } else {
                        (
                            current_state.current_coord.0,
                            current_state.current_coord.1 - 1,
                        )
                    }
                }
                Direction::R => {
                    if current_state.current_coord.1 == grid[0].len() - 1 {
                        continue;
                    } else {
                        (
                            current_state.current_coord.0,
                            current_state.current_coord.1 + 1,
                        )
                    }
                }
                Direction::Unspecified => panic!("next direction should not be unspecified"),
            };
            next_state.current_coord = new_coord;
            next_state.path.push(new_coord);
            next_state.push_direction(d);
            next_state.total_heat_loss +=
                grid[next_state.current_coord.0][next_state.current_coord.1];
            push_with_priority(&mut priority_queue, next_state);
        }

        seen_coords_with_paths.insert((current_state.current_coord, current_state.last_3_moves));
    }

    panic!("Did not find the end of grid.")
}

fn push_with_priority(queue: &mut VecDeque<SearchState>, new: SearchState) {
    match queue
        .iter()
        .position(|state| state.total_heat_loss > new.total_heat_loss)
    {
        Some(i) => {
            if i == 0 {
                queue.push_front(new)
            } else {
                queue.insert(i - 1, new)
            }
        }
        None => queue.push_back(new),
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct SearchStateUltraCrucible {
    last_10_moves: [Direction; 10],
    current_coord: (usize, usize),
    path: Vec<(usize, usize)>,
    total_heat_loss: u32,
}

impl SearchStateUltraCrucible {
    fn push_direction(&mut self, direction: Direction) {
        for i in 0..=8 {
            self.last_10_moves[i] = self.last_10_moves[i + 1];
        }
        self.last_10_moves[9] = direction;
    }
}

fn search_with_ultra_crucible(grid: &[Vec<u32>]) -> u32 {
    let end = (grid.len() - 1, grid[0].len() - 1);

    let mut priority_queue = VecDeque::<SearchStateUltraCrucible>::new();
    // Cache optimisation -> If we're at a coordinate with the same last directions - we are very likely in a cycle
    // of some kind - so use this to break out of loops.
    let mut seen_coords_with_paths = HashSet::<((usize, usize), [Direction; 10])>::new();

    let initial_search = SearchStateUltraCrucible {
        last_10_moves: [Direction::Unspecified; 10],
        current_coord: (0, 0),
        path: vec![(0, 0)],
        total_heat_loss: 0,
    };
    priority_queue.push_front(initial_search);

    while let Some(current_state) = priority_queue.pop_front() {
        if current_state.current_coord == end
            && current_state.last_10_moves[6..=9].iter().all_equal()
        {
            print_path_taken(grid, &current_state.path);
            return current_state.total_heat_loss;
        }
        if seen_coords_with_paths
            .contains(&(current_state.current_coord, current_state.last_10_moves))
        {
            continue;
        }

        // Same rules as before - except we how have to have a seqeunce of 4 before we are allowed to turn.
        let mut next_directions = HashSet::<Direction>::new();
        match current_state.last_10_moves[9] {
            Direction::U => {
                next_directions.insert(Direction::U);
                if current_state.last_10_moves[6..=9]
                    .iter()
                    .all(|d| *d == Direction::U)
                {
                    next_directions.insert(Direction::L);
                    next_directions.insert(Direction::R);
                }
            }
            Direction::D => {
                next_directions.insert(Direction::D);
                if current_state.last_10_moves[6..=9]
                    .iter()
                    .all(|d| *d == Direction::D)
                {
                    next_directions.insert(Direction::L);
                    next_directions.insert(Direction::R);
                }
            }
            Direction::L => {
                next_directions.insert(Direction::L);

                if current_state.last_10_moves[6..=9]
                    .iter()
                    .all(|d| *d == Direction::L)
                {
                    next_directions.insert(Direction::U);
                    next_directions.insert(Direction::D);
                }
            }
            Direction::R => {
                next_directions.insert(Direction::R);

                if current_state.last_10_moves[6..=9]
                    .iter()
                    .all(|d| *d == Direction::R)
                {
                    next_directions.insert(Direction::U);
                    next_directions.insert(Direction::D);
                }
            }
            Direction::Unspecified => {
                // this is the starting position - so only down and right are valid
                next_directions.insert(Direction::D);
                next_directions.insert(Direction::R);
            }
        }
        if current_state
            .last_10_moves
            .iter()
            .all(|d| *d == Direction::D)
        {
            next_directions.remove(&Direction::D);
        }
        if current_state
            .last_10_moves
            .iter()
            .all(|d| *d == Direction::U)
        {
            next_directions.remove(&Direction::U);
        }
        if current_state
            .last_10_moves
            .iter()
            .all(|d| *d == Direction::L)
        {
            next_directions.remove(&Direction::L);
        }
        if current_state
            .last_10_moves
            .iter()
            .all(|d| *d == Direction::R)
        {
            next_directions.remove(&Direction::R);
        }

        // Handle the new directions:
        for d in next_directions {
            let mut next_state = current_state.clone();
            let new_coord = match d {
                Direction::U => {
                    if current_state.current_coord.0 == 0 {
                        continue;
                    } else {
                        (
                            current_state.current_coord.0 - 1,
                            current_state.current_coord.1,
                        )
                    }
                }
                Direction::D => {
                    if current_state.current_coord.0 == grid.len() - 1 {
                        continue;
                    } else {
                        (
                            current_state.current_coord.0 + 1,
                            current_state.current_coord.1,
                        )
                    }
                }
                Direction::L => {
                    if current_state.current_coord.1 == 0 {
                        continue;
                    } else {
                        (
                            current_state.current_coord.0,
                            current_state.current_coord.1 - 1,
                        )
                    }
                }
                Direction::R => {
                    if current_state.current_coord.1 == grid[0].len() - 1 {
                        continue;
                    } else {
                        (
                            current_state.current_coord.0,
                            current_state.current_coord.1 + 1,
                        )
                    }
                }
                Direction::Unspecified => panic!("next direction should not be unspecified"),
            };
            next_state.current_coord = new_coord;
            next_state.path.push(new_coord);
            next_state.push_direction(d);
            next_state.total_heat_loss += grid[new_coord.0][new_coord.1];
            push_with_priority_ultra_crucible(&mut priority_queue, next_state);
        }

        seen_coords_with_paths.insert((current_state.current_coord, current_state.last_10_moves));
    }

    panic!("Did not find the end of grid.")
}

fn push_with_priority_ultra_crucible(
    queue: &mut VecDeque<SearchStateUltraCrucible>,
    new: SearchStateUltraCrucible,
) {
    match queue
        .iter()
        .position(|state| state.total_heat_loss > new.total_heat_loss)
    {
        Some(i) => {
            if i == 0 {
                queue.push_front(new)
            } else {
                queue.insert(i - 1, new)
            }
        }
        None => queue.push_back(new),
    }
}

#[allow(unused)]
fn print_path_taken(grid: &[Vec<u32>], path: &[(usize, usize)]) {
    println!();
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if path.contains(&(y, x)) {
                print!("X");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
