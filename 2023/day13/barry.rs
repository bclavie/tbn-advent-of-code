fn main() {
    let inputs = include_str!("inputs/13");
    let patterns: Vec<&str> = inputs.split("\n\n").collect();
    let reflections: Vec<Reflection> = patterns
        .iter()
        .map(|pattern| {
            let area: Vec<Vec<char>> = pattern.lines().map(|l| l.chars().collect()).collect();

            // Horizontal Patterns
            match find_reflection(&area) {
                Some(r) => Reflection::Horizontal(r),
                None => {
                    // Vertical Pattern just transpose and do horizontal again
                    let mut transposed_area = vec![vec!['x'; area.len()]; area[0].len()];
                    for j in 0..area.len() {
                        for (i, _) in area[0].iter().enumerate() {
                            transposed_area[i][j] = area[j][i];
                        }
                    }
                    match find_reflection(&transposed_area) {
                        Some(r) => Reflection::Vertical(r),
                        None => panic!("reflection not found"),
                    }
                }
            }
        })
        .collect();

    let part1: u64 = reflections.iter().map(|r| r.to_score()).sum();
    println!("part1: {}", part1);

    let part2_reflections: Vec<Reflection> = patterns
        .iter()
        .map(|pattern| {
            let area: Vec<Vec<char>> = pattern.lines().map(|l| l.chars().collect()).collect();
            // Horizontal Patterns
            match find_almost_reflection(&area) {
                Some(r) => Reflection::Horizontal(r),
                None => {
                    // Vertical Pattern just transpose and do horizontal again
                    let mut transposed_area = vec![vec!['x'; area.len()]; area[0].len()];
                    for j in 0..area.len() {
                        for (i, _) in area[0].iter().enumerate() {
                            transposed_area[i][j] = area[j][i];
                        }
                    }
                    match find_almost_reflection(&transposed_area) {
                        Some(r) => Reflection::Vertical(r),
                        None => panic!("reflection not found"),
                    }
                }
            }
        })
        .collect();
    let part2: u64 = part2_reflections.iter().map(|r| r.to_score()).sum();
    println!("part2: {}", part2);
}

fn find_reflection(area: &[Vec<char>]) -> Option<usize> {
    for reflection_point in 1..area.len() {
        let mut reflection_found = true;
        for i in 1..=reflection_point {
            if (reflection_point + i - 1) >= area.len() {
                break;
            }
            if area[reflection_point - i] != area[reflection_point + i - 1] {
                reflection_found = false;
                break;
            }
        }
        if reflection_found {
            return Some(reflection_point);
        }
    }
    None
}

fn find_almost_reflection(area: &[Vec<char>]) -> Option<usize> {
    for reflection_point in 1..area.len() {
        let mut total_differences = 0;
        for i in 1..=reflection_point {
            if (reflection_point + i - 1) >= area.len() {
                break;
            }
            total_differences +=
                lines_differ(&area[reflection_point - i], &area[reflection_point + i - 1]);
            if total_differences > 1 {
                break;
            }
        }
        if total_differences == 1 {
            return Some(reflection_point);
        }
    }
    None
}

fn lines_differ(l1: &[char], l2: &[char]) -> i64 {
    let mut num_diff = 0;
    for i in 0..l1.len() {
        if l1[i] != l2[i] {
            num_diff += 1;
        }
    }
    num_diff
}

#[derive(Debug)]
enum Reflection {
    Vertical(usize),
    Horizontal(usize),
}

impl Reflection {
    fn to_score(&self) -> u64 {
        match self {
            Reflection::Vertical(i) => *i as u64,
            Reflection::Horizontal(i) => *i as u64 * 100,
        }
    }
}
