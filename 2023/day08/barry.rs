use std::collections::HashMap;

fn main() {
    let inputs = include_str!("inputs/08");
    let (instructions, nodes) = inputs.split_once("\n\n").unwrap();
    let map = Nodes::from(nodes);

    let mut current = "AAA";
    let mut steps_req = 0;
    for (steps, i) in instructions.chars().cycle().enumerate() {
        let current_map = map.mappings.get(current).unwrap();
        current = match i {
            'L' => &current_map.0,
            'R' => &current_map.1,
            _ => panic!("unexpected instruction"),
        };
        if current == "ZZZ" {
            steps_req = steps + 1;
            break;
        }
    }

    // part 1
    println!("{}", steps_req);

    // part 2
    let mut current_nodes: Vec<String> = map
        .mappings
        .keys()
        .filter_map(|s| {
            if s.ends_with('A') {
                let node = s.clone();
                Some(node)
            } else {
                None
            }
        })
        .collect();
    // for each starting loc - when do we start cycling landing on a "Z".
    let mut z_seen_before: Vec<bool> = vec![false; current_nodes.len()];
    let mut steps_for_first_z: Vec<i64> = vec![i64::MAX; current_nodes.len()];
    let mut step_for_last_seen_z: Vec<i64> = vec![i64::MAX; current_nodes.len()];
    let mut z_cycle: Vec<i64> = vec![i64::MAX; current_nodes.len()];
    let mut z_cycle2: Vec<i64> = vec![i64::MAX; current_nodes.len()];
    for (steps, i) in instructions.chars().cycle().enumerate() {
        current_nodes = current_nodes
            .iter()
            .map({
                |current| {
                    let current_map = map.mappings.get(current).unwrap();
                    let new = match i {
                        'L' => &current_map.0,
                        'R' => &current_map.1,
                        _ => panic!("unexpected instruction"),
                    };
                    new.into()
                }
            })
            .collect();

        for (j, node) in current_nodes.iter().enumerate() {
            if node.ends_with('Z') {
                if !z_seen_before[j] {
                    steps_for_first_z[j] = steps as i64 + 1;
                    z_seen_before[j] = true;
                } else {
                    if z_cycle[j] == i64::MAX {
                        z_cycle[j] = steps as i64 + 1 - steps_for_first_z[j];
                    } else {
                        z_cycle2[j] = steps as i64 + 1 - (step_for_last_seen_z[j]);
                    }

                    step_for_last_seen_z[j] = steps as i64 + 1;
                }
            }
        }
        if z_cycle2.iter().all(|z| z < &i64::MAX) {
            break;
        }
    }
    // Assert that each cycle is actually consistent.
    for (i, step) in z_cycle2.iter().enumerate() {
        debug_assert_eq!(*step, z_cycle[i]);
    }
    // I got lucky here, not sure if it's guaranteed that both my first steps and cycle lengths were consistent.
    println!("cycle lengths: {:?}", z_cycle2);
    let lcm = z_cycle2.into_iter().reduce(lcm).unwrap();
    println!("part2: {}", lcm)
}

struct Nodes {
    // node to L/R string.
    mappings: HashMap<String, (String, String)>,
}

impl From<&str> for Nodes {
    fn from(value: &str) -> Self {
        let mut mappings = HashMap::<String, (String, String)>::new();
        for line in value.lines() {
            let (source, dests) = line.split_once(" = ").unwrap();
            let (dest_l, dest_r) = dests.split_once(", ").unwrap();

            mappings.insert(
                source.to_string(),
                (
                    dest_l[1..].to_string(),
                    dest_r[0..dest_r.len() - 1].to_string(),
                ),
            );
        }
        Self { mappings }
    }
}

fn lcm(x: i64, y: i64) -> i64 {
    x * y / gcd(x,y)
}

fn gcd(x: i64, y: i64) -> i64 {
    let start = if y < x { y } else { x };
    for i in (1..start).rev() {
        if y % i == 0 && x % i == 0 {
            return i;
        }
    }
    panic!("not found");
}
