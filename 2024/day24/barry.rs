use std::collections::HashMap;

fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/inputs/24"));

    let mut wires = HashMap::<String, u8>::new();

    let (init, gates) = input.split_once("\n\n").unwrap();
    for line in init.lines() {
        let (name, value) = line.split_once(": ").unwrap();
        let value = value.parse().unwrap();

        wires.insert(name.to_string(), value);
    }

    let all_gates: HashMap<&str, Gate> = gates
        .lines()
        .map(|l| {
            let gate = Gate::from_input(l);
            (gate.target, gate)
        })
        .collect();

    let mut z_gates: Vec<String> = all_gates
        .iter()
        .filter_map(|(k, _)| {
            if k.starts_with('z') {
                Some(k.to_string())
            } else {
                None
            }
        })
        .collect();
    z_gates.sort();

    let part1 = get_z_value(&wires, &all_gates, &z_gates);
    println!("part1 {}", part1);

    // Part 2 - how do we find which wires are wrong?
    // Firstly I assume this is "just a binary adder".
    // https://www.electronics-tutorials.ws/combination/comb_7.html
    // So each zn should have one XOR going into it, if it isn't
    // then it needs to be swapped with something.
    // one should be the direct output of xn and yn.
    // the other should be the XOR of the carry of xn-1 yn-1 along with
    // the XOR output of xn and yn.
    // This gives us one quick gate that is dodgy: fsp AND bdr -> z39
    //
    // How do we find the rest?
    // First pass - iterate through adding 0b1, 0b10 ... 2^44 to 0.
    // This lets you see which outputs might be suspect.
    let mut all_gates_with_swaps = all_gates.clone();
    // 05 swaps
    all_gates_with_swaps.entry("nbc").and_modify(|gate| {
        gate.left = "x05";
        gate.right = "y05";
        gate.t = GateType::And;
    });

    all_gates_with_swaps.entry("svm").and_modify(|gate| {
        gate.left = "x05";
        gate.right = "y05";
        gate.t = GateType::Xor;
    });

    // 39 swaps
    all_gates_with_swaps.entry("z39").and_modify(|gate| {
        gate.left = "bdr";
        gate.right = "fsp";
        gate.t = GateType::Xor;
    });

    all_gates_with_swaps.entry("fnr").and_modify(|gate| {
        gate.left = "x39";
        gate.right = "y39";
        gate.t = GateType::And;
    });

    // 15 swaps
    all_gates_with_swaps.entry("z15").and_modify(|gate| {
        gate.left = "fwr";
        gate.right = "cpv";
        gate.t = GateType::Xor;
    });

    all_gates_with_swaps.entry("kqk").and_modify(|gate| {
        gate.left = "dkk";
        gate.right = "pbd";
        gate.t = GateType::Or;
    });

    // 23 swaps
    all_gates_with_swaps.entry("z23").and_modify(|gate| {
        gate.left = "kph";
        gate.right = "hpw";
        gate.t = GateType::Xor;
    });

    all_gates_with_swaps.entry("cgq").and_modify(|gate| {
        gate.left = "x23";
        gate.right = "y23";
        gate.t = GateType::And;
    });

    let mut incorrect_bits_identity = vec![];

    // Testing identity on each x bit.
    for i in 0..=44 {
        let mut test_wires = HashMap::new();
        for j in 0..=44 {
            let x_val = format!("x{:0>2}", j);
            let y_val = format!("y{:0>2}", j);

            if j == i {
                test_wires.insert(x_val, 1);
                test_wires.insert(y_val, 0);
            } else {
                test_wires.insert(x_val, 0);
                test_wires.insert(y_val, 0);
            }
        }
        let result = get_z_value(&test_wires, &all_gates_with_swaps, &z_gates);
        let expected = 1 << (i);
        if result != expected {
            println!("unexpected result for identity on idx {}, expected: {}, bin: {:#b}, actual: {}, {:#b}", i, expected, expected, result, result);
            incorrect_bits_identity.push(i);
        }
    }

    println!("incorrect_bits_identity: {:?}", incorrect_bits_identity);

    // Then trying inputs for carry with simple addition.
    let mut incorrect_bits_addition = vec![];
    for i in 0..=44 {
        let mut test_wires = HashMap::new();
        for j in 0..=44 {
            let x_val = format!("x{:0>2}", j);
            let y_val = format!("y{:0>2}", j);

            if j == i {
                test_wires.insert(x_val, 1);
                test_wires.insert(y_val, 1);
            } else {
                test_wires.insert(x_val, 0);
                test_wires.insert(y_val, 0);
            }
        }
        let result = get_z_value(&test_wires, &all_gates_with_swaps, &z_gates);
        let expected = 1 << (i + 1);
        if result != expected {
            println!("unexpected result for addition on idx {}, expected: {}, bin: {:#b}, actual: {}, {:#b}", i, expected, expected, result, result);
            incorrect_bits_addition.push(i);
        }
    }

    println!("incorrect_bits_addition: {:?}", incorrect_bits_addition);

    let mut incorrect_bits_carry = vec![];
    // Testing complex carrying on each bit.
    for i in 0..=44 {
        let mut test_wires = HashMap::new();
        for j in 0..=44 {
            let x_val = format!("x{:0>2}", j);
            let y_val = format!("y{:0>2}", j);

            if j <= i {
                test_wires.insert(x_val, 1);
                test_wires.insert(y_val, 1);
            } else {
                test_wires.insert(x_val, 0);
                test_wires.insert(y_val, 0);
            }
        }

        let result = get_z_value(&test_wires, &all_gates_with_swaps, &z_gates);
        let mut expected = !0;
        expected >>= 63 - i;
        expected <<= 1;

        if result != expected {
            println!("unexpected result for carry on idx {}, expected: {}, bin: {:#b}, actual: {}, {:#b}", i, expected, expected, result, result);
            incorrect_bits_carry.push(i);
        }
    }

    println!("incorrect_bits_carry: {:?}", incorrect_bits_carry);

    // Manual inspection gets us the answer!
}

fn get_z_value(
    values: &HashMap<String, u8>,
    gates: &HashMap<&str, Gate>,
    z_names: &[String],
) -> u64 {
    let mut z_values = vec![];
    for z in z_names.iter() {
        z_values.push(traverse(values, gates, z));
    }

    let mut part1 = 0;
    for (shifts, z) in z_values.into_iter().enumerate() {
        let target: u64 = (z as u64) << shifts;
        part1 += target;
    }
    part1
}


fn traverse(values: &HashMap<String, u8>, gates: &HashMap<&str, Gate>, gate: &str) -> u8 {
    let current_gate = gates.get(gate).unwrap();

    let left = match values.get(current_gate.left) {
        Some(l) => *l,
        None => traverse(values, gates, current_gate.left),
    };

    let right = match values.get(current_gate.right) {
        Some(r) => *r,
        None => traverse(values, gates, current_gate.right),
    };

    match current_gate.t {
        GateType::And => left & right,
        GateType::Xor => left ^ right,
        GateType::Or => left | right,
    }
}

#[derive(Clone, Copy, Debug)]
struct Gate<'a> {
    t: GateType,
    left: &'a str,
    right: &'a str,
    #[allow(unused)]
    target: &'a str,
}

impl<'a> Gate<'a> {
    fn from_input(input: &'a str) -> Self {
        let (head, target) = input.split_once(" -> ").unwrap();
        let mut iter = head.split(' ');
        let left = iter.next().unwrap();
        let t = GateType::from_str(iter.next().unwrap());
        let right = iter.next().unwrap();

        Self {
            t,
            left,
            right,
            target,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum GateType {
    And,
    Xor,
    Or,
}

impl GateType {
    fn from_str(s: &str) -> Self {
        match s {
            "AND" => Self::And,
            "XOR" => Self::Xor,
            "OR" => Self::Or,
            _ => panic!("unexpected"),
        }
    }
}
