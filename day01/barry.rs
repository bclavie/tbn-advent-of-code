fn main() {
    // part 1
    let inputs = include_str!("inputs/01");

    let calibration_numbers: Vec<i64>= inputs
        .split('\n')
        .map(calibration_number).collect();

    println!("part 1: {}", calibration_numbers.into_iter().sum::<i64>());

    // part2
    let calibration_numbers_2: Vec<i64> = inputs.split('\n').map(
        calibration_p2
    ).collect();

    println!("part 2: {}", calibration_numbers_2.into_iter().sum::<i64>());
}

fn calibration_number(s: &str) -> i64 {
    let mut chars = s.chars().filter(|c| c.is_ascii_digit());
    let first: char = chars.next().unwrap();
    let last = match chars.last() {
        Some(num) => num,
        None => first,
    };
    let mut num_string = first.to_string();
    num_string.push(last);

    num_string.parse::<i64>().unwrap()
}

fn calibration_p2(s: &str) -> i64 {
    // Good old for loops
    let chars: Vec<char> = s.chars().collect();
    let mut first: char = ' ';
    for i in 0..chars.len() {
        match get_digit(&chars[i..]){
            Some(digit) => {
                first = digit;
                break;
            },
            None => {
                continue
            }
        }
    }

    let mut last: char = ' ';
    for i in (0..chars.len()).rev() {
        match get_digit(&chars[i..]){
            Some(digit) => {
                last = digit;
                break;
            },
            None => {
                continue
            }
        }
    }
    let mut num_string = first.to_string();
    num_string.push(last);

    num_string.parse::<i64>().unwrap()
}

fn get_digit(slice: &[char])  -> Option<char> {
    match slice {
        ['1', ..] => Some('1'),
        ['2', ..] => Some('2'),
        ['3', ..] => Some('3'),
        ['4', ..] => Some('4'),
        ['5', ..] => Some('5'),
        ['6', ..] => Some('6'),
        ['7', ..] => Some('7'),
        ['8', ..] => Some('8'),
        ['9', ..] => Some('9'),
        ['o', 'n', 'e', ..] => Some('1'),
        ['t', 'w', 'o', ..] => Some('2'),
        ['t', 'h', 'r', 'e', 'e', ..] => Some('3'),
        ['f', 'o', 'u', 'r', ..] => Some('4'),
        ['f', 'i', 'v', 'e', ..] => Some('5'),
        ['s', 'i', 'x', ..] => Some('6'),
        ['s', 'e', 'v', 'e', 'n', ..] => Some('7'),
        ['e', 'i', 'g', 'h', 't', ..] => Some('8'),
        ['n', 'i', 'n', 'e', ..] => Some('9'),
        _ => None,
    }
}