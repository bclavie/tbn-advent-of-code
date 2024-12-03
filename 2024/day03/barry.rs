fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/inputs/3"));

    let char_vec: Vec<char> = input.chars().collect();
    let mut mul_vec: Vec<Mul> = vec![];
    for i in 0..char_vec.len() {
        match Mul::parse_char(&char_vec[i..]) {
            Some(m) => mul_vec.push(m),
            None => continue,
        }
    }

    let part1 = mul_vec.iter().fold(0, |acc, m| acc + m.mul());
    println!("part1 {}", part1);

    let mut mul_vec_enabled: Vec<Mul> = vec![];
    let mut is_enabled = true;
    for i in 0..char_vec.len() {
        if let Some(enabled) = parse_enabled(&char_vec[i..]) {
            is_enabled = enabled;
        }

        if is_enabled {
            match Mul::parse_char(&char_vec[i..]) {
                Some(m) => mul_vec_enabled.push(m),
                None => continue,
            }
        }
    }

    let part2 = mul_vec_enabled.iter().fold(0, |acc, m| acc + m.mul());
    println!("part2 {}", part2);
}

struct Mul {
    left: i64,
    right: i64,
}

impl Mul {
    // Parses a char array from the beginning maybe returning a Mul.
    fn parse_char(chars: &[char]) -> Option<Mul> {
        if chars.len() < 8 {
            return None;
        }

        // consume mul
        if chars[0..=2] != ['m', 'u', 'l'] {
            return None;
        }

        if chars[3] != '(' {
            return None;
        }

        let mut current_idx = 4;
        let left: i64;
        match parse_number(&chars[current_idx..]) {
            Some((num, consumed)) => {
                left = num;
                current_idx += consumed;
            }
            None => return None,
        }

        if current_idx >= chars.len() || chars[current_idx] != ',' {
            return None;
        }

        current_idx += 1;

        let right :i64;
        match parse_number(&chars[current_idx..]) {
            Some((num, consumed)) => {
                right = num;
                current_idx += consumed;
            }
            None => return None,
        }

        if current_idx >= chars.len() || chars[current_idx] != ')' {
            return None;
        }

        Some(Mul { left, right })
    }

    fn mul(&self) -> i64 {
        self.left * self.right
    }
}

// Returns the number and the number of digits consumed.
fn parse_number(chars: &[char]) -> Option<(i64, usize)> {
    let mut numbers = Vec::<i64>::new();
    for c in chars {
        match c.to_digit(10) {
            Some(dig) => numbers.push(dig.into()),
            None => break,
        }
    }

    if numbers.is_empty() {
        return None;
    }

    let mut value = 0;
    for (pow, num) in numbers.iter().rev().enumerate() {
        value += num * 10_i64.pow(pow.try_into().unwrap())
    }

    Some((value, numbers.len()))
}

fn parse_enabled(chars: &[char]) -> Option<bool> {
    match chars {
        ['d', 'o', '(', ')', ..] => Some(true),
        ['d', 'o', 'n', '\'', 't', '(', ')', ..] => Some(false),
        _ => None,
    }
}
