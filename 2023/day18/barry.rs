use itertools::Itertools;

fn main() {
    let inputs = include_str!("inputs/18");
    let instructions: Vec<Instruction> = inputs.lines().map(Instruction::from_str).collect();

    // Part 2 - > completely fucks the model for p1 so this got re-written.
    // After some googling: https://en.wikipedia.org/wiki/Pick%27s_theorem + https://en.wikipedia.org/wiki/Shoelace_formula

    let mut current_vertex = (0, 0);
    let mut corners = vec![];
    for inst in instructions.iter() {
        match inst.direction {
            Direction::U => current_vertex.0 += (inst.amount) as i64,
            Direction::D => current_vertex.0 -= (inst.amount) as i64,
            Direction::L => current_vertex.1 -= (inst.amount) as i64,
            Direction::R => current_vertex.1 += (inst.amount) as i64,
        }
        corners.push(current_vertex);
    }
    let part1 = area_theorem(&corners, &instructions);
    println!("part1: {}", part1);

    let mut current_vertex2 = (0, 0);
    let mut corners2 = vec![];
    let real_instructions: Vec<Instruction> = instructions
        .iter()
        .map(|i| {
            let hex_str = i.hex.as_str();
            let direction = match &hex_str[hex_str.len() - 1..hex_str.len()] {
                "0" => Direction::R,
                "1" => Direction::D,
                "2" => Direction::L,
                "3" => Direction::U,
                x => panic!("unexpected direction: {}", x),
            };
            let amount = u64::from_str_radix(&hex_str[0..hex_str.len() - 1], 16).unwrap();

            Instruction {
                amount,
                direction,
                hex: "".to_string(),
            }
        })
        .collect();
    for inst in real_instructions.iter() {
        match inst.direction {
            Direction::U => current_vertex2.0 += (inst.amount) as i64,
            Direction::D => current_vertex2.0 -= (inst.amount) as i64,
            Direction::L => current_vertex2.1 -= (inst.amount) as i64,
            Direction::R => current_vertex2.1 += (inst.amount) as i64,
        }
        corners2.push(current_vertex2);
    }
    let part2 = area_theorem(&corners2, &real_instructions);
    println!("part2: {}", part2);
}

// Picks theorem is:
// Area = Sum of Interior + (Sum of boundary points)/2 - 1;
// Shoelace gets you a way to get area. 
// The problem asks you to count the total number of points inside and on the boundary ( which is the perimeter value).
// Area + 1 = sum of interior + (sum of boundary) / 2
// Area + 1 + sum of boundary / 2 = sum of interior + sum of boundary = solution.
fn area_theorem(corners: &[(i64, i64)], instructions: &[Instruction]) -> u64 {
    // Shoelace
    let mut sum_of_determinants: i64 = 0;

    for i in 0..corners.len() {
        if i != corners.len() - 1 {
            sum_of_determinants +=
                (corners[i].0 * corners[i + 1].1) - (corners[i].1 * corners[i + 1].0);
        } else {
            sum_of_determinants += (corners[i].0 * corners[0].1) - (corners[i].1 * corners[0].0);
        }
    }

    // Make sure this area is a whole number since our coordinates are only integers.
    debug_assert!((sum_of_determinants % 2) == 0);
    let area: u64 = (sum_of_determinants.abs() / 2).try_into().unwrap();

    let mut perimeter_length = 0;
    for i in instructions {
        perimeter_length += i.amount;
    }

    area + perimeter_length / 2 + 1
}

struct Instruction {
    direction: Direction,
    amount: u64,
    hex: String,
}

impl Instruction {
    fn from_str(s: &str) -> Self {
        let (direction_str, amount_str, hex_str) = s.splitn(3, ' ').collect_tuple().unwrap();

        Self {
            direction: Direction::from_str(direction_str),
            amount: amount_str.parse().unwrap(),
            hex: hex_str[2..hex_str.len() - 1].to_string(),
        }
    }
}

enum Direction {
    U,
    D,
    L,
    R,
}

impl Direction {
    fn from_str(s: &str) -> Self {
        match s {
            "U" => Self::U,
            "D" => Self::D,
            "L" => Self::L,
            "R" => Self::R,
            _ => panic!("unexpected str"),
        }
    }
}
