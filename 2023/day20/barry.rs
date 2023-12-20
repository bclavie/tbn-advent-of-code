use std::collections::{HashMap, VecDeque};

fn main() {
    let input = include_str!("inputs/20");

    let mut graph = Graph::new();
    for l in input.lines() {
        let prefix = l.chars().next().unwrap();
        let (name_str, targets) = l.split_once(" -> ").unwrap();
        let neighbours = targets.split(", ").map(|t| t.to_string()).collect();

        let (name, node) = match prefix {
            '%' => (
                &name_str[1..name_str.len()],
                Node::FlipFlop {
                    neighbours,
                    is_on: false,
                },
            ),
            '&' => (
                &name_str[1..name_str.len()],
                Node::Conjunction {
                    neighbours,
                    memory: HashMap::new(),
                },
            ),
            'b' => ("broadcaster", Node::BroadCaster { neighbours }),
            _ => panic!("Unexpected node"),
        };

        graph.nodes.insert(name.to_string(), node);
    }
    graph.nodes.insert("button".to_string(), Node::Button);

    // One more pass to set conjunction nodes and to fill in test nodes...
    let graph_clone = graph.clone();
    for (k, v) in graph_clone.nodes {
        for n in v.neighbours() {
            // println!("{}", n);
            // There is a single node which is the output -> "output" in test, or "rx" in the real input.
            if graph.nodes.get(&n).is_none() {
                graph.nodes.insert(n.clone(), Node::Test);
                graph.feeds_rx = k.clone();
            }
            if let Node::Conjunction { memory, .. } = graph.nodes.get_mut(&n).unwrap() {
                memory.insert(k.clone(), Pulse::Low);
            }
        }
    }

    let mut graph2 = graph.clone();

    // Part 1
    let (mut total_low, mut total_high) = (0, 0);
    for i in 0..1000 {
        let (low, high) = graph.push_button(i);
        total_high += high;
        total_low += low;
    }

    let part1 = total_high * total_low;
    println!("part1: {}", part1);

    // Part2 - get the LCM of the iterations it takes for each 
    // inputs into the node before RX (a conjunction node)
    // to be high. The LCM of this is the first iteration where they will
    // all be high, hence RX will get a low pulse.
    let mut i = 0;
    loop {
        i += 1;
        graph2.push_button(i);
        if graph2.has_seen_all_inputs_to_rx() {
            break;
        }
    }
    let mut part2 = 1;
    for v in graph2.first_instance_of_high.values() {
        part2 = lcm(part2, *v);
    }

    println!("part2: {}", part2);
}

fn lcm(x: u64, y: u64) -> u64 {
    x * y / gcd(x, y)
}

fn gcd(x: u64, y: u64) -> u64 {
    if x == y {
        return x;
    }
    let start = if y < x { y } else { x };
    for i in (1..=start).rev() {
        if y % i == 0 && x % i == 0 {
            return i;
        }
    }
    panic!("not found");
}

#[derive(Debug, Clone)]
struct Graph {
    nodes: HashMap<String, Node>,
    feeds_rx: String,
    first_instance_of_high: HashMap<String, u64>,
}
impl Graph {
    fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            feeds_rx: "".to_string(),
            first_instance_of_high: HashMap::new(),
        }
    }

    fn has_seen_all_inputs_to_rx(&self) -> bool {
        let feed_rx_node = self.nodes.get(&self.feeds_rx).unwrap();
        match feed_rx_node {
            Node::Conjunction { memory, .. } => {
                for k in memory.keys() {
                    if !self.first_instance_of_high.contains_key(k) {
                        return false;
                    }
                }
                true
            }
            _ => panic!("invalid node type"),
        }
    }

    fn push_button(&mut self, i: u64) -> (u64, u64) {
        // from, to, pulse type
        let mut pulse_queue: VecDeque<(String, String, Pulse)> =
            VecDeque::<(String, String, Pulse)>::new();
        let mut num_low = 0;
        let mut num_high: u64 = 0;

        pulse_queue.push_back(("me".to_string(), "button".to_string(), Pulse::Low));
        while let Some(pulse) = pulse_queue.pop_front() {
            // Cache the first instance of i where a particular input of the node
            // before RX is high
            if pulse.1 == self.feeds_rx
                && pulse.2 == Pulse::High
                && !self.first_instance_of_high.contains_key(&pulse.0)
            {
                self.first_instance_of_high.insert(pulse.0.clone(), i);
            }
            let node = self.nodes.get_mut(&pulse.1).unwrap();

            let (new_pulses, low, high) = node.handle_pulse(pulse.0, pulse.2);
            num_high += high;
            num_low += low;
            for p in new_pulses {
                // println!("{} -{:?} -> {}", pulse.1.clone(), p.1, p.0);
                pulse_queue.push_back((pulse.1.clone(), p.0, p.1))
            }
        }

        (num_low, num_high)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug, Clone)]
enum Node {
    Button,
    FlipFlop {
        neighbours: Vec<String>,
        is_on: bool,
    },
    Conjunction {
        neighbours: Vec<String>,
        memory: HashMap<String, Pulse>,
    },
    BroadCaster {
        neighbours: Vec<String>,
    },
    Test,
}

impl Node {
    fn neighbours(&self) -> Vec<String> {
        match self {
            Node::Button => vec!["broadcaster".to_string()],
            Node::FlipFlop { neighbours, .. } => neighbours.clone(),
            Node::Conjunction { neighbours, .. } => neighbours.clone(),
            Node::BroadCaster { neighbours } => neighbours.clone(),
            Node::Test => vec![],
        }
    }

    // The new pulses, num low and num high.
    fn handle_pulse(&mut self, from: String, pulse: Pulse) -> (Vec<(String, Pulse)>, u64, u64) {
        let mut pulses = vec![];
        let mut low = 0;
        let mut high = 0;
        match self {
            Node::Button => {
                low += 1;
                pulses.push(("broadcaster".to_string(), Pulse::Low));
            }
            Node::FlipFlop { neighbours, is_on } => match pulse {
                Pulse::High => {}
                Pulse::Low => {
                    if *is_on {
                        for n in neighbours {
                            low += 1;
                            pulses.push((n.clone(), Pulse::Low));
                        }
                    } else {
                        for n in neighbours {
                            high += 1;
                            pulses.push((n.clone(), Pulse::High));
                        }
                    }
                    *is_on = !*is_on;
                }
            },
            Node::Conjunction { neighbours, memory } => {
                let pulse = memory.insert(from, pulse);
                debug_assert!(pulse.is_some());

                // println!("{:?}", memory);
                for n in neighbours {
                    if memory.values().all(|p| *p == Pulse::High) {
                        pulses.push((n.clone(), Pulse::Low));
                        low += 1;
                    } else {
                        high += 1;
                        pulses.push((n.clone(), Pulse::High));
                    }
                }
            }
            Node::BroadCaster { neighbours } => {
                for n in neighbours {
                    match pulse {
                        Pulse::High => high += 1,
                        Pulse::Low => low += 1,
                    }
                    pulses.push((n.clone(), pulse));
                }
            }
            Node::Test => {}
        };

        (pulses, low, high)
    }
}
