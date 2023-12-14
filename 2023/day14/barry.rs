use std::collections::HashMap;

fn main() {
    let inputs = include_str!("inputs/14");
    let mut grid = Grid::from_str(inputs);
    let mut grid2 = grid.clone();
    grid.model_fall();
    grid.print_grid();

    let part1 = grid.get_total_scores();

    println!("part1: {}", part1);

    grid2.model_cycle();
    let part2 = grid2.get_total_scores();

    println!("part2: {}", part2);
}

#[derive(Clone)]
struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn print_grid(&self) {
        println!();
        for g in self.grid.iter() {
            for c in g {
                print!("{}", c);
            }
            println!()
        }
    }

    fn from_str(s: &str) -> Self {
        let grid = s.lines().map(|l| l.chars().collect()).collect();
        Self { grid }
    }

    fn model_fall(&mut self) {
        // Have to do this len times to model all drops correctly..
        for i in (0..self.grid.len()).rev() {
            for k in 0..=i {
                if k > 0 {
                    for j in 0..self.grid[k].len() {
                        if self.grid[k][j] == 'O' && self.grid[k - 1][j] == '.' {
                            self.grid[k][j] = '.';
                            self.grid[k - 1][j] = 'O';
                        }
                    }
                }
            }
        }
    }

    // Rotates 90 degrees clockwise so we can model the cycles: n -> w > s > e
    fn rotate(&mut self) {
        let mut new_grid = vec![vec![' '; self.grid[0].len()]; self.grid.len()];
        let row_len = self.grid[0].len();
        for i in 0..self.grid.len() {
            for (j, _) in self.grid[0].iter().enumerate() {
                // A rotation is swapping rows and cols.
                new_grid[j][row_len - i - 1] = self.grid[i][j];
            }
        }
        self.grid = new_grid;
    }

    fn model_cycle(&mut self) {
        let mut state = HashMap::<Vec<Vec<char>>, usize>::new();
        let mut current_cycle_num = 0;
        loop {
            for _ in 0..4 {
                self.model_fall();
                self.rotate();
            }
            current_cycle_num += 1;
            if let Some(seen) = state.get(&self.grid) {
                // diff is the cycle length.
                let diff = current_cycle_num - seen;
                // if this is the one that would be the 1000000000th then break out.
                if (1000000000 - current_cycle_num) % diff == 0 {
                    break;
                }
            } else {
                state.insert(self.grid.clone(), current_cycle_num);
            }
        }
    }

    fn get_total_scores(&self) -> u64 {
        self.grid
            .iter()
            .rev()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter().map(move |c| match c {
                    'O' => (i + 1) as u64,
                    _ => 0,
                })
            })
            .sum()
    }
}
