use std::collections::{BinaryHeap, HashMap};

fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/inputs/21"));
    let inputs: Vec<Input<'_>> = input.lines().map(Input::from_input).collect();

    let number_locations = HashMap::<char, (usize, usize)>::from([
        ('7', (0, 0)),
        ('8', (1, 0)),
        ('9', (2, 0)),
        ('4', (0, 1)),
        ('5', (1, 1)),
        ('6', (2, 1)),
        ('1', (0, 2)),
        ('2', (1, 2)),
        ('3', (2, 2)),
        ('.', (0, 3)),
        ('0', (1, 3)),
        ('A', (2, 3)),
    ]);

    let keypad_locations = HashMap::<char, (usize, usize)>::from([
        ('.', (0, 0)),
        ('^', (1, 0)),
        ('A', (2, 0)),
        ('<', (0, 1)),
        ('v', (1, 1)),
        ('>', (2, 1)),
    ]);

    let mut cost_cache = HashMap::new();
    let mut part1 = 0;
    let mut part2 = 0;
    for input in &inputs {
        part1 += solve(
            &number_locations,
            &keypad_locations,
            2,
            &mut cost_cache,
            input,
        ) as i64
            * input.numeric;
        part2 += solve(
            &number_locations,
            &keypad_locations,
            25,
            &mut cost_cache,
            input,
        ) as i64
            * input.numeric;
    }
    println!("part1: {}", part1);
    println!("part2: {}", part2);
}

struct Input<'a> {
    raw: &'a str,
    numeric: i64,
}

impl<'a> Input<'a> {
    fn from_input(line: &'a str) -> Self {
        Self {
            raw: line,
            numeric: line.strip_suffix('A').unwrap().parse().unwrap(),
        }
    }

    fn char_vec(&self) -> Vec<char> {
        self.raw.chars().collect()
    }
}

#[derive(Eq, PartialEq)]
struct QueueElementNumpad {
    cost: usize,
    current_node: (usize, usize),
    target_idx: usize,
    previous_char: char,
    found: bool,
}

impl Ord for QueueElementNumpad {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for QueueElementNumpad {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// cost cache is used for keypad grid to determine the cost of doing a move on the given level.
fn solve(
    number_grid: &HashMap<char, (usize, usize)>,
    keypad_grid: &HashMap<char, (usize, usize)>,
    total_nesting: usize,
    cost_cache: &mut HashMap<(char, char, usize), usize>,
    input: &Input,
) -> usize {
    // We do bfs on the number grid and keep a track of the cost using the cost of the move based on the total nesting level. (2 in part1, 25 in part2).
    let start = number_grid.get(&'A').unwrap();
    let panic_node = number_grid.get(&'.').unwrap();

    // Priority Queue based on current cost.
    let mut queue = BinaryHeap::from([QueueElementNumpad {
        cost: 0,
        current_node: *start,
        target_idx: 0,
        previous_char: 'A',
        found: false,
    }]);
    while let Some(QueueElementNumpad {
        cost,
        current_node,
        target_idx,
        previous_char,
        found,
    }) = queue.pop()
    {
        if found {
            return cost;
        }
        // If we're at the end, get the cost of pressing 'A' - but we still need to insert it back into the queue, as
        // the cost of pressing A might make this path more expensive than another path.
        if target_idx == 3 && current_node == *start {
            let new_cost =
                cost + keypad_cost(keypad_grid, cost_cache, previous_char, 'A', total_nesting);
            queue.push(QueueElementNumpad {
                cost: new_cost,
                current_node,
                target_idx,
                previous_char: 'A',
                found: true,
            });
            continue;
        }
        if current_node == *panic_node {
            continue;
        }
        let target = input.char_vec()[target_idx];
        let target_node = number_grid.get(&target).unwrap();

        // If we're at the current target node, we need to press 'A' before moving to the next one.
        if *target_node == current_node {
            let new_cost =
                cost + keypad_cost(keypad_grid, cost_cache, previous_char, 'A', total_nesting);

            queue.push(QueueElementNumpad {
                cost: new_cost,
                current_node,
                target_idx: target_idx + 1,
                previous_char: 'A',
                found,
            });
            continue;
        }

        // move left
        if target_node.0 < current_node.0 {
            let new_location = (current_node.0 - 1, current_node.1);
            let new_cost =
                cost + keypad_cost(keypad_grid, cost_cache, previous_char, '<', total_nesting);

            queue.push(QueueElementNumpad {
                cost: new_cost,
                current_node: new_location,
                target_idx,
                previous_char: '<',
                found,
            });
        }

        // right
        if target_node.0 > current_node.0 {
            let new_location = (current_node.0 + 1, current_node.1);
            let new_cost =
                cost + keypad_cost(keypad_grid, cost_cache, previous_char, '>', total_nesting);

            queue.push(QueueElementNumpad {
                cost: new_cost,
                current_node: new_location,
                target_idx,
                previous_char: '>',
                found,
            });
        }

        // up
        if target_node.1 < current_node.1 {
            let new_location = (current_node.0, current_node.1 - 1);
            let new_cost =
                cost + keypad_cost(keypad_grid, cost_cache, previous_char, '^', total_nesting);

            queue.push(QueueElementNumpad {
                cost: new_cost,
                current_node: new_location,
                target_idx,
                previous_char: '^',
                found,
            });
        }

        // down
        if target_node.1 > current_node.1 {
            let new_location = (current_node.0, current_node.1 + 1);
            let new_cost =
                cost + keypad_cost(keypad_grid, cost_cache, previous_char, 'v', total_nesting);

            queue.push(QueueElementNumpad {
                cost: new_cost,
                current_node: new_location,
                target_idx,
                previous_char: 'v',
                found,
            });
        }
    }

    unreachable!();
}

#[derive(Eq, PartialEq)]
struct QueueElementKeypad {
    cost: usize,
    current_node: (usize, usize),
    previous_char: char,
    hit_target: bool,
}

impl Ord for QueueElementKeypad {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for QueueElementKeypad {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn keypad_cost(
    keypad_grid: &HashMap<char, (usize, usize)>,
    cost_cache: &mut HashMap<(char, char, usize), usize>,
    start: char,
    end: char,
    levels_remaining: usize,
) -> usize {
    // Levels remaining 0 == human level so inputs are just easy size 1.
    if levels_remaining == 0 {
        return 1;
    }

    // Every duplicate entry is just a press of 'A' at all levels.
    if start == end {
        return 1;
    }

    if let Some(cost) = cost_cache.get(&(start, end, levels_remaining)) {
        return *cost;
    }

    let panic_node = keypad_grid.get(&'.').unwrap();
    // We do bfs here as well. But we have to recurse into the costs for the next level.
    let start_node = keypad_grid.get(&start).unwrap();
    let target_node = keypad_grid.get(&end).unwrap();
    // Every search starts at 'A' since you needed to have pressed 'A' in the next level to register the previous button.
    let mut queue = BinaryHeap::from([QueueElementKeypad {
        cost: 0,
        current_node: *start_node,
        previous_char: 'A',
        hit_target: false,
    }]);
    while let Some(QueueElementKeypad {
        cost,
        current_node,
        previous_char,
        hit_target,
    }) = queue.pop()
    {
        if current_node == *panic_node {
            continue;
        }
        // If we're at the target node, press 'A' - but we still need to insert it into the queue
        // as the act of pressing A might increase the cost more than another path.
        if *target_node == current_node {
            if !hit_target {
                let new_cost = cost
                    + keypad_cost(
                        keypad_grid,
                        cost_cache,
                        previous_char,
                        'A',
                        levels_remaining - 1,
                    );
                queue.push(QueueElementKeypad {
                    cost: new_cost,
                    current_node,
                    previous_char: 'A',
                    hit_target: true,
                });
                continue;
            }
            // Finally return cache and return the cost;
            cost_cache.insert((start, end, levels_remaining), cost);
            return cost;
        }

        // Otherwise, move closer to the target and accumulate the cost using the movement.

        // move left
        if target_node.0 < current_node.0 {
            let new_location: (usize, usize) = (current_node.0 - 1, current_node.1);
            let new_cost = cost
                + keypad_cost(
                    keypad_grid,
                    cost_cache,
                    previous_char,
                    '<',
                    levels_remaining - 1,
                );

            queue.push(QueueElementKeypad {
                cost: new_cost,
                current_node: new_location,
                previous_char: '<',
                hit_target: false,
            });
        }

        // right
        if target_node.0 > current_node.0 {
            let new_location = (current_node.0 + 1, current_node.1);
            let new_cost = cost
                + keypad_cost(
                    keypad_grid,
                    cost_cache,
                    previous_char,
                    '>',
                    levels_remaining - 1,
                );

            queue.push(QueueElementKeypad {
                cost: new_cost,
                current_node: new_location,
                previous_char: '>',
                hit_target: false,
            });
        }

        // up
        if target_node.1 < current_node.1 {
            let new_location = (current_node.0, current_node.1 - 1);
            let new_cost = cost
                + keypad_cost(
                    keypad_grid,
                    cost_cache,
                    previous_char,
                    '^',
                    levels_remaining - 1,
                );

            queue.push(QueueElementKeypad {
                cost: new_cost,
                current_node: new_location,
                previous_char: '^',
                hit_target: false,
            });
        }

        // down
        if target_node.1 > current_node.1 {
            let new_location = (current_node.0, current_node.1 + 1);
            let new_cost = cost
                + keypad_cost(
                    keypad_grid,
                    cost_cache,
                    previous_char,
                    'v',
                    levels_remaining - 1,
                );

            queue.push(QueueElementKeypad {
                cost: new_cost,
                current_node: new_location,
                previous_char: 'v',
                hit_target: false,
            });
        }
    }

    unreachable!();
}
