use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

fn main() {
    let input = include_str!("inputs/23")
        .lines()
        .map(|l| l.chars().collect_vec())
        .collect_vec();
    let graph = Graph::from_input(&input);
    let start_x = input[0].iter().enumerate().find(|c| *c.1 == '.').unwrap();
    let end_x = input[input.len() - 1]
        .iter()
        .enumerate()
        .find(|c| *c.1 == '.')
        .unwrap();

    let start = (0, start_x.0);
    let end = (input.len() - 1, end_x.0);
    let paths = graph.find_all_paths(start, end);
    println!("part 1 {}", paths.iter().max().unwrap());

    // Part 2 is a bit scuffed, it takes a while to run but it finished executing
    // before I finished optimising my solution further.
    let graph_part2 = Graph::from_input_part2(&input);
    let paths2 = graph_part2.find_all_paths_part2(start, end);
    println!("part 2 {}", paths2.iter().max().unwrap());
}

struct Graph {
    nodes: HashMap<(usize, usize), Node>,
}

impl Graph {
    fn from_input_part2(input: &[Vec<char>]) -> Self {
        let mut nodes = HashMap::new();
        for (y, line) in input.iter().enumerate() {
            for (x, point) in line.iter().enumerate() {
                if ['.', '^', '>', '<', 'v'].contains(point) {
                    let mut node = Node {
                        c: *point,
                        neighbours: vec![],
                    };
                    if y > 0 && input[y - 1][x] != '#' {
                        node.neighbours.push((y - 1, x));
                    }
                    if y < input.len() - 1 && input[y + 1][x] != '#' {
                        node.neighbours.push((y + 1, x));
                    }

                    if x > 0 && input[y][x - 1] != '#' {
                        node.neighbours.push((y, x - 1));
                    }
                    if x < input[0].len() - 1 && input[y][x + 1] != '#' {
                        node.neighbours.push((y, x + 1));
                    }

                    nodes.insert((y, x), node);
                }
            }
        }

        Self { nodes }
    }

    fn from_input(input: &[Vec<char>]) -> Self {
        let mut nodes = HashMap::new();
        for (y, line) in input.iter().enumerate() {
            for (x, point) in line.iter().enumerate() {
                match point {
                    '.' => {
                        let mut node = Node {
                            c: *point,
                            neighbours: vec![],
                        };
                        if y > 0 && input[y - 1][x] != '#' {
                            node.neighbours.push((y - 1, x));
                        }
                        if y < input.len() - 1 && input[y + 1][x] != '#' {
                            node.neighbours.push((y + 1, x));
                        }

                        if x > 0 && input[y][x - 1] != '#' {
                            node.neighbours.push((y, x - 1));
                        }
                        if x < input[0].len() - 1 && input[y][x + 1] != '#' {
                            node.neighbours.push((y, x + 1));
                        }

                        nodes.insert((y, x), node);
                    }
                    '^' => {
                        let mut node = Node {
                            c: *point,
                            neighbours: vec![],
                        };
                        if y > 0 && input[y - 1][x] != '#' {
                            node.neighbours.push((y - 1, x));
                        }
                        nodes.insert((y, x), node);
                    }
                    '>' => {
                        let mut node = Node {
                            c: *point,
                            neighbours: vec![],
                        };
                        if x < input[0].len() - 1 && input[y][x + 1] != '#' {
                            node.neighbours.push((y, x + 1));
                        }

                        nodes.insert((y, x), node);
                    }
                    'v' => {
                        let mut node = Node {
                            c: *point,
                            neighbours: vec![],
                        };
                        if y < input.len() - 1 && input[y + 1][x] != '#' {
                            node.neighbours.push((y + 1, x));
                        }

                        nodes.insert((y, x), node);
                    }

                    '<' => {
                        let mut node = Node {
                            c: *point,
                            neighbours: vec![],
                        };
                        if x > 0 && input[y][x - 1] != '#' {
                            node.neighbours.push((y, x - 1));
                        }
                        nodes.insert((y, x), node);
                    }
                    _ => {}
                }
            }
        }

        Self { nodes }
    }

    fn find_all_paths(&self, start: (usize, usize), end: (usize, usize)) -> Vec<u64> {
        let mut paths = vec![];

        // Current node, hashset of visited.
        let mut queue = VecDeque::<((usize, usize), HashSet<(usize, usize)>)>::new();
        let mut starting_set = HashSet::new();
        starting_set.insert(start);
        queue.push_back((start, starting_set));

        while let Some(current) = queue.pop_front() {
            if current.0 == end {
                // minus 1 for start
                paths.push(current.1.len() as u64 - 1);
                continue;
            }

            for neighbour in &self.nodes.get(&current.0).unwrap().neighbours {
                if current.1.contains(neighbour) {
                    continue;
                }
                let mut new_hash = current.1.clone();
                new_hash.insert(*neighbour);
                queue.push_back((*neighbour, new_hash));
            }
        }

        paths
    }

    // DFS based approach for speed, trying to optimise away the single choice nodes.
    fn find_all_paths_part2(&self, start: (usize, usize), end: (usize, usize)) -> Vec<usize> {
        self._dfs_inner(start, end, HashSet::new())
    }

    fn _dfs_inner(
        &self,
        start: (usize, usize),
        end: (usize, usize),
        visited: HashSet<(usize, usize)>,
    ) -> Vec<usize> {
        let mut length = 0;
        let mut lens = vec![];
        let mut visited = visited;
        let mut current = start;
        let mut options: Vec<&(usize, usize)>;

        loop {
            let current_node = self.nodes.get(&current).unwrap();
            // If we dont have any choices, get traverse the only other option.
            options = current_node
                .neighbours
                .iter()
                .filter(|n| !visited.contains(n))
                .collect_vec();
            length += 1;

            if options.len() == 1 {
                visited.insert(*options[0]);
                current = *options[0];
                if current == end {
                    lens.push(length);
                    return lens;
                }
            } else {
                break;
            }
        }

        let mut option_lengths = vec![];
        for o in options {
            let mut new_visited = visited.clone();
            new_visited.insert(*o);
            let lengths = self._dfs_inner(*o, end, new_visited);
            option_lengths.extend(lengths);
        }

        for l in option_lengths.iter_mut() {
            lens.push(*l + length)
        }

        lens
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Node {
    // Neighbour nodes
    c: char,
    neighbours: Vec<(usize, usize)>,
}
