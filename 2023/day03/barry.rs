use std::collections::HashMap;

fn main() {
    // part 1
    let inputs = include_str!("inputs/03");

    let schematic: Vec<Vec<Input>> = inputs
        .split('\n')
        .map(|line| line.chars().map(Input::from).collect())
        .collect();
    // mark the numbers
    let mut nums_adjacent_to_symbols: HashMap<(usize, usize), i64> =
        HashMap::<(usize, usize), i64>::new();
    for (i, row) in schematic.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            match schematic[i][j] {
                Input::Symbol(_) | Input::Gear => {
                    // if it's not a digit or symbol, check for adjacent nums.
                    if i > 0 && j > 0 {
                        mark_number(&mut nums_adjacent_to_symbols, i - 1, j - 1, &schematic);
                    }
                    if j > 0 {
                        mark_number(&mut nums_adjacent_to_symbols, i, j - 1, &schematic);
                        mark_number(&mut nums_adjacent_to_symbols, i + 1, j - 1, &schematic);
                    }
                    if i > 0 {
                        mark_number(&mut nums_adjacent_to_symbols, i - 1, j, &schematic);
                        mark_number(&mut nums_adjacent_to_symbols, i - 1, j + 1, &schematic);
                    }

                    mark_number(&mut nums_adjacent_to_symbols, i + 1, j, &schematic);
                    mark_number(&mut nums_adjacent_to_symbols, i, j + 1, &schematic);
                    mark_number(&mut nums_adjacent_to_symbols, i + 1, j + 1, &schematic);
                }
                _ => {}
            }
        }
    }
    let mut actual_marked_nums = Vec::<i64>::new();
    // Do a 2nd pass operate on the schematic to combine marked numbers into their actual values e.g. 4 and 3 should be a single 43 and then
    // append it to the sum to take into account.
    for (i, _) in schematic.clone().iter().enumerate() {
        let mut j = 0;
        while j < schematic[i].len() {
            if let Input::Number(num) = schematic[i][j] {
                let mut recorded_num = num;
                let mut js_to_merge = Vec::<usize>::new();
                js_to_merge.push(j);

                while let Some(Input::Number(next_num)) = schematic[i].get(j + 1) {
                    js_to_merge.push(j + 1);
                    recorded_num = recorded_num * 10 + next_num;
                    j += 1;
                }

                if js_to_merge
                    .iter()
                    .map(|k| nums_adjacent_to_symbols.get(&(i, *k)))
                    .any(|found| found.is_some())
                {
                    actual_marked_nums.push(recorded_num)
                }
            }
            j += 1;
        }
    }

    let sum: i64 = actual_marked_nums.iter().sum();
    println!("part1: {}\n", sum);
    // part 2
    println!("part2: {}", part2(&schematic))
}

fn part2(schematic: &[Vec<Input>]) -> i64 {
    let mut gear_scores: Vec<i64> = Vec::<i64>::new();
    for (i, row) in schematic.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if let Input::Gear = schematic[i][j] {
                let mut adj_num_count = 0;
                // x, y, direction to look
                let mut adj_num_look_ups = Vec::<(usize, usize, LookDirection)>::new();

                // First detect any adjacent numbers
                // This assumes that in the input we only get 3 digit numbers.
                // If we have any numbers to the left or right - we know that is a complete number.
                // Looking above and below is trickier. If any '.' occurs in middle in the line above or below
                // then there's a number to each diagonal.
                // Look Left
                if j > 0 && is_number(schematic, i, j - 1) {
                    adj_num_count += 1;
                    adj_num_look_ups.push((i, j - 1, LookDirection::Left))
                }
                // Look Right
                if is_number(schematic, i, j + 1) {
                    adj_num_count += 1;
                    adj_num_look_ups.push((i, j + 1, LookDirection::Right))
                }
                // Look above
                if i > 0 {
                    // All 3
                    if j > 0
                        && is_number(schematic, i - 1, j - 1)
                        && is_number(schematic, i - 1, j)
                        && is_number(schematic, i - 1, j + 1)
                    {
                        adj_num_count += 1;
                        adj_num_look_ups.push((i - 1, j - 1, LookDirection::Right))
                    }
                    // left & middle
                    else if j > 0
                        && is_number(schematic, i - 1, j - 1)
                        && is_number(schematic, i - 1, j)
                    {
                        adj_num_count += 1;
                        adj_num_look_ups.push((i - 1, j, LookDirection::Left))
                    }
                    // left and right
                    else if j > 0
                        && is_number(schematic, i - 1, j - 1)
                        && is_number(schematic, i - 1, j + 1)
                    {
                        adj_num_count += 2;
                        adj_num_look_ups.push((i - 1, j - 1, LookDirection::Left));
                        adj_num_look_ups.push((i - 1, j + 1, LookDirection::Right));
                    }
                    // middle & right
                    else if is_number(schematic, i - 1, j) && is_number(schematic, i - 1, j + 1) {
                        adj_num_count += 1;
                        adj_num_look_ups.push((i - 1, j, LookDirection::Right))
                    }
                    // Just left
                    else if j > 0 && is_number(schematic, i - 1, j - 1) {
                        adj_num_count += 1;
                        adj_num_look_ups.push((i - 1, j - 1, LookDirection::Left));
                    }
                    // Just Right
                    else if is_number(schematic, i - 1, j + 1) {
                        adj_num_count += 1;
                        adj_num_look_ups.push((i - 1, j + 1, LookDirection::Right));
                    }
                    // Just above (single digit case)
                    else if is_number(schematic, i-1, j) {
                        adj_num_count += 1;
                        adj_num_look_ups.push((i - 1, j, LookDirection::Right));
                    }
                }
                // Look Below
                // All 3
                if j > 0
                    && is_number(schematic, i + 1, j - 1)
                    && is_number(schematic, i + 1, j)
                    && is_number(schematic, i + 1, j + 1)
                {
                    adj_num_count += 1;
                    adj_num_look_ups.push((i + 1, j - 1, LookDirection::Right))
                }
                // left & middle
                else if j > 0
                    && is_number(schematic, i + 1, j - 1)
                    && is_number(schematic, i + 1, j)
                {
                    adj_num_count += 1;
                    adj_num_look_ups.push((i + 1, j, LookDirection::Left))
                }
                // left and right
                else if j > 0
                    && is_number(schematic, i + 1, j - 1)
                    && is_number(schematic, i + 1, j + 1)
                {
                    adj_num_count += 2;
                    adj_num_look_ups.push((i + 1, j - 1, LookDirection::Left));
                    adj_num_look_ups.push((i + 1, j + 1, LookDirection::Right));
                }
                // middle & right
                else if is_number(schematic, i + 1, j) && is_number(schematic, i + 1, j + 1) {
                    adj_num_count += 1;
                    adj_num_look_ups.push((i + 1, j, LookDirection::Right))
                }
                // Just left
                else if j > 0 && is_number(schematic, i + 1, j - 1) {
                    adj_num_count += 1;
                    adj_num_look_ups.push((i + 1, j - 1, LookDirection::Left));
                }
                // Just Right
                else if is_number(schematic, i + 1, j + 1) {
                    adj_num_count += 1;
                    adj_num_look_ups.push((i + 1, j + 1, LookDirection::Right));
                }
                // Just below (single digit case)
                else if is_number(schematic, i+1, j) {
                    adj_num_count += 1;
                    adj_num_look_ups.push((i + 1, j, LookDirection::Right));
                }

                // Are there only two numbers?
                if adj_num_count == 2 {
                    let adj_nums: Vec<i64> = adj_num_look_ups
                        .into_iter()
                        .map(|x| match x.2 {
                            LookDirection::Left => find_num_looking_left(schematic, x.0, x.1),
                            LookDirection::Right => find_num_looking_right(schematic, x.0, x.1),
                        })
                        .collect();

                    println!("adj nums: {:?}", adj_nums);
                    let gear_power: i64 = adj_nums.iter().product();
                    gear_scores.push(gear_power)
                }
            }
        }
    }

    gear_scores.iter().sum()
}

enum LookDirection {
    Left,
    Right,
}

fn is_number(schematic: &[Vec<Input>], i: usize, j: usize) -> bool {
    matches!(
        schematic.get(i).and_then(|r| r.get(j)),
        Some(Input::Number(_))
    )
}

fn find_num_looking_right(schematic: &[Vec<Input>], i: usize, j: usize) -> i64 {
    let mut j = j;
    let mut recorded_num = match schematic[i].get(j).unwrap() {
        Input::Number(n) => *n,
        _ => {
            eprint!("{}, {}, {:?}", i, j, schematic[i][j]);
            panic!("unexpected non-number")
        }
    };

    while let Some(Input::Number(next_num)) = schematic[i].get(j + 1) {
        recorded_num = recorded_num * 10 + next_num;
        j += 1;
    }

    recorded_num
}

fn find_num_looking_left(schematic: &[Vec<Input>], i: usize, j: usize) -> i64 {
    let mut j = j;
    let mut recorded_num = match schematic[i].get(j).unwrap() {
        Input::Number(n) => *n,
        _ => {
            eprint!("{}, {}, {:?}", i, j, schematic[i][j]);
            panic!("unexpected non-number")
        }
    };
    let mut ten_factor = 10;
    while j > 0 {
        if let Some(Input::Number(next_num)) = schematic[i].get(j - 1) {
            recorded_num += ten_factor * next_num;
            ten_factor *= 10;
            j -= 1;
        } else {
            break;
        }
    }

    recorded_num
}

#[derive(Clone, Debug)]
enum Input {
    Number(i64),
    Symbol(char),
    Gear,
    Nothing,
}

impl From<char> for Input {
    fn from(value: char) -> Self {
        match value {
            value if value.is_ascii_digit() => Input::Number(value.to_digit(10).unwrap() as i64),
            '.' => Input::Nothing,
            '*' => Input::Gear,
            _ => Input::Symbol(value),
        }
    }
}

impl TryFrom<Input> for i64 {
    type Error = String;

    fn try_from(value: Input) -> Result<Self, Self::Error> {
        if let Input::Number(i) = value {
            Ok(i)
        } else {
            Err("Not a number".to_string())
        }
    }
}

fn mark_number(
    map: &mut HashMap<(usize, usize), i64>,
    i: usize,
    j: usize,
    schematic: &[Vec<Input>],
) {
    if let Some(Input::Number(n)) = schematic.get(i).and_then(|r| r.get(j)) {
        map.insert((i, j), *n);
    }
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part2_inputs() {
        // Sample
        let inputs: Vec<(&str, i64)> = vec![(r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#, 467835),
// Both all 3
(r#"890
.*.
904"#, 804560),
// Left and Right
(r#"890
.*.
904"#, 804560),
// Two diagonals
(r#"890....
...*...
....904"#, 804560),
(r#"....890
...*...
904...."#, 804560),
// Single digits
(r#"...3....
...*...
...7..."#, 21),
// Left and Right above
(r#"571.342
...*...
......."#, 195282),
];

        for input in inputs {
            let schematic: Vec<Vec<Input>> = input.0
            .split('\n')
            .map(|line| line.chars().map(Input::from).collect())
            .collect();
        let answer = part2(&schematic);

            assert_eq!(answer, input.1)
        }

    }
    
}