fn main() {
    let inputs = include_str!("inputs/15");
    let hash_numbers: Vec<u64> = inputs.split(',').map(hash).collect();

    let part1: u64 = hash_numbers.iter().sum();
    println!("part1: {}", part1);

    let mut part2_hashmap = ScuffedHashMap::new();
    let part2_inputs = inputs.split(',').map(Instruction::from_str);

    for i in part2_inputs {
        part2_hashmap.do_instruction(i)
    }

    let part2 = part2_hashmap.get_score();
    println!("part2: {}", part2);
}

fn hash(s: &str) -> u64 {
    let mut value = 0;
    for c in s.chars() {
        if !c.is_ascii() {
            panic!("unexpected char")
        }
        value += c as u64;
        value *= 17;
        value %= 256;
    }
    value
}

struct Instruction {
    label: String,
    op: Operation,
    value: u64,
}

impl Instruction {
    fn from_str(s: &str) -> Self {
        if s.ends_with('-') {
            Self {
                label: s[0..s.len() - 1].to_string(),
                op: Operation::Remove,
                value: 0,
            }
        } else {
            let (label, value) = s.split_once('=').unwrap();
            Self {
                label: label.to_string(),
                op: Operation::Set,
                value: value.parse().unwrap(),
            }
        }
    }
}

enum Operation {
    Set,
    Remove,
}

struct ScuffedHashMap {
    boxes: [Vec<Lens>; 256],
}

#[derive(Clone, Debug)]
struct Lens {
    label: String,
    value: u64,
}

impl ScuffedHashMap {
    fn new() -> Self {
        let boxes: [Vec<Lens>; 256] = vec![vec![]; 256].try_into().unwrap();
        Self { boxes }
    }

    fn do_instruction(&mut self, instruction: Instruction) {
        let lens = Lens {
            label: instruction.label.clone(),
            value: instruction.value,
        };
        let box_num = hash(&instruction.label) as usize;
        let b = &mut self.boxes[box_num];
        match instruction.op {
            Operation::Set => {
                let mut has_found = false;
                for l in &mut *b {
                    if l.label == lens.label {
                        *l = lens.clone();
                        has_found = true;
                        break;
                    }
                }
                if !has_found {
                    b.push(lens)
                }
            }
            Operation::Remove => {
                for i in 0..b.len() {
                    if b[i].label == lens.label {
                        b.remove(i);
                        break;
                    }
                }
            }
        }
    }

    fn get_score(&self) -> u64 {
        let mut total = 0;
        for (box_index, b) in self.boxes.iter().enumerate() {
            for (lens_index, lens) in b.iter().enumerate() {
                let focal_power = (box_index as u64 + 1) * (lens_index as u64 + 1) * lens.value;
                total += focal_power;
            }
        }
        total
    }
}
