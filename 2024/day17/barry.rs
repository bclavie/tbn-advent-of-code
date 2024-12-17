fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/inputs/17"));

    let mut program = Program::from_str(input);
    let instructions = program
        .instructions
        .clone()
        .iter()
        .map(|u| *u)
        .collect::<Vec<u8>>();

    let part1 = program.execute();
    println!("part1 {:?}", part1);

    // Basically from inspection - there is a pattern that we can abuse. And all the instructions seem to operate on
    // integers modulo 8, we can split our input into 3 bit vals. If we have e.g a 3 bit val:
    // x1x2..xn has output a1,a2....,an
    // then the input
    // x1x2,,xny will
    // have output Y,a1,a2..,an
    // where Y is within 0->7.
    // so we can just do a dfs using the lowest input that currently matches the end of the desired output.
    let part2 = part_2_dfs(&instructions, &mut program, 0).unwrap();
    println!("part2: {}", part2)
}

fn part_2_dfs(target: &[u8], program: &mut Program, current: i64) -> Option<i64> {
    let nums = [0, 1, 2, 3, 4, 5, 6, 7];

    for n in nums {
        let test = current + n;
        program.reset(test);
        let out = program.execute();
        if out.len() > target.len() {
            return None;
        }

        if target == out {
            return Some(test);
        }
        if target.ends_with(&out) {
            let next = test << 3;
            if next == 0 {
                continue;
            }
            if let Some(answer) = part_2_dfs(target, program, next) {
                return Some(answer);
            }
        }
    }

    None
}

struct Program {
    register_a: i64,
    register_b: i64,
    register_c: i64,
    instructions: Vec<u8>,
    current_instruction_idx: usize,
}

impl Program {
    fn from_str(input: &str) -> Self {
        let (registers, program) = input.split_once("\n\n").unwrap();
        let mut registers = registers.split('\n');
        let register_a = registers
            .next()
            .unwrap()
            .strip_prefix("Register A: ")
            .unwrap()
            .parse()
            .unwrap();
        let register_b = registers
            .next()
            .unwrap()
            .strip_prefix("Register B: ")
            .unwrap()
            .parse()
            .unwrap();
        let register_c = registers
            .next()
            .unwrap()
            .strip_prefix("Register C: ")
            .unwrap()
            .parse()
            .unwrap();

        let program = program.strip_prefix("Program: ").unwrap();
        let instructions = program.split(',').map(|s| s.parse().unwrap()).collect();

        Self {
            register_a,
            register_b,
            register_c,
            instructions,
            current_instruction_idx: 0,
        }
    }

    fn get_combo_operand(&self, val: u8) -> i64 {
        match val {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => {
                panic!("unexpected combo operand")
            }
        }
    }

    fn reset(&mut self, register_a: i64) {
        self.register_a = register_a;
        self.register_b = 0;
        self.register_c = 0;
        self.current_instruction_idx = 0;
    }

    fn execute(&mut self) -> Vec<u8> {
        let mut out: Vec<u8> = vec![];

        while self.current_instruction_idx < self.instructions.len() {
            let opcode = self.instructions[self.current_instruction_idx];
            let operand = self.instructions[self.current_instruction_idx + 1];
            match opcode {
                // adv
                0 => {
                    let denominator = 2_i64
                        .checked_pow(self.get_combo_operand(operand).try_into().unwrap())
                        .unwrap();
                    self.register_a = self.register_a / denominator;
                }
                //bxl
                1 => {
                    let literal_operand = operand as i64;
                    self.register_b = self.register_b ^ literal_operand;
                }
                //bst
                2 => {
                    let combo_operand = self.get_combo_operand(operand) % 8;
                    self.register_b = combo_operand;
                }
                // jnz
                3 => {
                    if self.register_a != 0 {
                        self.current_instruction_idx = operand as usize;
                        continue;
                    }
                }
                // bxc
                4 => {
                    self.register_b = self.register_b ^ self.register_c;
                }
                // out
                5 => {
                    let combo_operand = self.get_combo_operand(operand) % 8;
                    out.push(combo_operand.try_into().unwrap());
                }
                // bdv
                6 => {
                    let denominator = 2_i64
                        .checked_pow(self.get_combo_operand(operand).try_into().unwrap())
                        .unwrap();
                    self.register_b = self.register_a / denominator;
                }
                // cdv
                7 => {
                    let denominator = 2_i64
                        .checked_pow(self.get_combo_operand(operand).try_into().unwrap())
                        .unwrap();
                    self.register_c = self.register_a / denominator;
                }
                _ => {
                    panic!("unexpected")
                }
            };

            self.current_instruction_idx += 2;
        }

        out
    }
}
