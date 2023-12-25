use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

fn main() {
    // Input has been reduced manually by visualising using GraphViz and inspecting & removing the
    // links:
    // sds -> hbr
    // dqf -> cbx
    // pzv -> xft
    // So we can just count the subgraph of nodes from sds and hbr to get the two answers.
    let inputs = include_str!("inputs/25");

    let graph = Graph::from_input(inputs);
    let part1 = graph.count_sub_graph("sds") * graph.count_sub_graph("hbr");

    println!("part1: {}", part1);
}

struct Graph {
    nodes: HashMap<String, Node>,
}

impl Graph {
    fn from_input(s: &str) -> Self {
        let mut nodes = HashMap::new();
        // jqt: rhn xhk nvd
        for l in s.lines() {
            let (node_str, neighbours_str) = l.split_once(": ").unwrap();
            let node = nodes.entry(node_str.to_string()).or_insert(Node {
                name: node_str.to_string(),
                neighbours: HashSet::new(),
            });

            let neighbours = neighbours_str.split(' ').collect_vec();

            // Need two loops otherwise the borrow checker is unhappy.
            for n in neighbours.iter() {
                node.neighbours.insert(n.to_string());
            }

            for n in neighbours {
                let neighbour_node = nodes.entry(n.to_string()).or_insert(Node {
                    name: n.to_string(),
                    neighbours: HashSet::new(),
                });
                neighbour_node.neighbours.insert(node_str.to_string());
            }
        }

        Self { nodes }
    }

    fn count_sub_graph(&self, start: &str) -> usize {
        let mut visited = HashSet::new();
        let start_node = self.nodes.get(start).unwrap();

        let mut queue = VecDeque::new();
        visited.insert(start);
        queue.push_back(start_node);

        while let Some(node) = queue.pop_front() {
            visited.insert(&node.name);
            for n in &node.neighbours {
                if !visited.contains(n.as_str()) {
                    let neighbour_node = self.nodes.get(n).unwrap();
                    queue.push_back(neighbour_node);
                }
            }
        }

        visited.len()
    }
}

struct Node {
    name: String,
    neighbours: HashSet<String>,
}

#[derive(Debug, Clone, Eq)]
struct Edge {
    left: String,
    right: String,
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left && self.right == other.right
            || self.left == other.right && self.right == other.left
    }
}
