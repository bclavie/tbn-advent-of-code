use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/inputs/23"));
    let graph = Graph::from_input(input);

    let part1 = part_1(&graph);
    println!("part1: {}", part1);

    let part2 = part_2(&graph);
    println!("part2: {}", part2);
}

#[derive(Debug, Clone)]
struct Graph {
    nodes: HashMap<String, Node>,
}

impl Graph {
    fn from_input(input: &str) -> Self {
        let mut nodes = HashMap::<String, Node>::new();
        for l in input.lines() {
            let (left, right) = l.split_once('-').unwrap();

            add_neighbour(
                nodes.entry(left.to_string()).or_default(),
                right.to_string(),
            );

            add_neighbour(
                nodes.entry(right.to_string()).or_default(),
                left.to_string(),
            );
        }

        Self { nodes }
    }
}

fn part_1(graph: &Graph) -> i64 {
    let mut three_sets = HashSet::<(String, String, String)>::new();
    for name in graph.nodes.keys() {
        if name.starts_with('t') {
            let sets = get_3_set(graph, name.clone());
            for s in sets {
                three_sets.insert(s);
            }
        }
    }
    three_sets.len() as i64
}

fn get_3_set(graph: &Graph, start: String) -> HashSet<(String, String, String)> {
    let mut three_sets = HashSet::<(String, String, String)>::new();
    // For each neighbour, check that that neighbour's neighours exist in our set.
    let start_node = graph.nodes.get(&start).unwrap();
    for neighbour in &start_node.neighbours {
        for neighbours_neighbour in &graph.nodes.get(neighbour).unwrap().neighbours {
            if *neighbours_neighbour == start {
                continue;
            }
            if start_node.neighbours.contains(neighbours_neighbour) {
                let mut three_set = [
                    start.clone(),
                    neighbour.clone(),
                    neighbours_neighbour.clone(),
                ];
                three_set.sort_unstable();
                three_sets.insert(three_set.into());
            }
        }
    }

    three_sets
}

fn part_2(graph: &Graph) -> String {
    let mut n = 1;
    let mut max_n_set = vec![];
    loop {
        if let Some(max_set) = get_n_set(graph, n) {
            max_n_set = max_set;
        } else {
            max_n_set.sort();
            return max_n_set.join(",");
        }
        n += 1;
    }
}

// get a connected set of atleast size n, doesn't matter where we start.
fn get_n_set(graph: &Graph, size: usize) -> Option<Vec<String>> {
    // current set (sorted alphabetically).
    let mut seen = HashSet::<Vec<String>>::new();
    let mut queue = VecDeque::new();

    for node in graph.nodes.keys() {
        let path = vec![node.clone()];
        queue.push_back((node.clone(), path));
    }

    while let Some((current, path)) = queue.pop_front() {
        if seen.contains(&path) {
            continue;
        }
        seen.insert(path.clone());

        if path.len() == size {
            return Some(path);
        }

        let current_node = graph.nodes.get(&current).unwrap();
        'neighbour_loop: for neighbour in &current_node.neighbours {
            if !path.contains(neighbour) {
                let neighbour_node = graph.nodes.get(neighbour).unwrap();

                for n in path.iter() {
                    if !neighbour_node.neighbours.contains(n) {
                        continue 'neighbour_loop;
                    }
                }
                let mut new_path = path.clone();
                new_path.push(neighbour.clone());
                new_path.sort();
                queue.push_back((neighbour.clone(), new_path));
            }
        }
    }

    None
}

fn add_neighbour(node: &mut Node, other: String) {
    node.neighbours.insert(other);
}

#[derive(Debug, Clone)]
struct Node {
    neighbours: HashSet<String>,
}

impl Node {
    fn new() -> Self {
        Self {
            neighbours: HashSet::new(),
        }
    }
}

impl Default for Node {
    fn default() -> Self {
        Self::new()
    }
}
