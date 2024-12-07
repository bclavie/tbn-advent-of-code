fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/inputs/7"));
    let inputs: Vec<Input> = input.lines().map(Input::from_str).collect();

    let (mut part1, mut part2) = (0, 0);
    for i in inputs {
        if i.is_truthy(i.nums[0], 1, false) {
            part1 += i.target;
        }
        if i.is_truthy(i.nums[0], 1, true) {
            part2 += i.target;
        }
    }

    println!("part1: {}", part1);
    println!("part2: {}", part2);
}

struct Input {
    target: i64,
    nums: Vec<i64>,
}

impl Input {
    fn from_str(s: &str) -> Self {
        let (target_str, other) = s.split_once(": ").unwrap();

        let target = target_str.parse().unwrap();
        let nums = other.split(' ').map(|s| s.parse().unwrap()).collect();

        Self { target, nums }
    }

    fn is_truthy(&self, current: i64, index: usize, include_concat: bool) -> bool {
        if current > self.target || index >= self.nums.len() {
            return false;
        }

        let addition_current = current + self.nums[index];
        if index == self.nums.len() - 1 && addition_current == self.target {
            return true;
        }

        if self.is_truthy(addition_current, index + 1, include_concat) {
            return true;
        }

        let multiplication_current = current * self.nums[index];
        if index == self.nums.len() - 1 && multiplication_current == self.target {
            return true;
        }
        if self.is_truthy(multiplication_current, index + 1, include_concat) {
            return true;
        }

        let concatination_current = concat(current, self.nums[index]);
        if index == self.nums.len() - 1 && concatination_current == self.target {
            return true;
        }
        if self.is_truthy(concatination_current, index + 1, include_concat) {
            return true;
        }

        false
    }
}

fn concat(x: i64, y: i64) -> i64 {
    (x.to_string() + &y.to_string()).parse().unwrap()
}
