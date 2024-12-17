use std::collections::HashSet;

fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/inputs/15"));
    let mut grid = Grid::from_str(input);
    grid.execute_moves();
    grid.print();

    let part1 = grid.gps();
    println!("part1: {}", part1);

    let mut grid2 = Grid::from_str_part2(input);
    grid2.execute_moves2();
    grid2.print();
    let part2 = grid2.gps2();
    println!("part2: {}", part2);
}

#[derive(Clone)]
struct Grid {
    grid: Vec<Vec<char>>,
    robot: (usize, usize),
    moves: Vec<Move>,
}

impl Grid {
    fn print(&self) {
        for l in &self.grid {
            for c in l {
                print!("{}", c);
            }
            println!();
        }
    }

    fn from_str_part2(input: &str) -> Self {
        let (grid, moves) = input.split_once("\n\n").unwrap();
        let grid: Vec<Vec<char>> = grid.lines().map(|l| l.chars().collect()).collect();

        let mut expanded_grid = vec![vec!['.'; 2 * grid[0].len()]; grid.len()];

        let mut robot = (0, 0);
        for j in 0..grid.len() {
            for i in 0..grid[0].len() {
                match grid[j][i] {
                    '@' => {
                        robot = (2 * i, j);
                        expanded_grid[j][2 * i] = '@';
                        expanded_grid[j][2 * i + 1] = '.';
                    }
                    '#' => {
                        expanded_grid[j][2 * i] = '#';
                        expanded_grid[j][2 * i + 1] = '#';
                    }
                    'O' => {
                        expanded_grid[j][2 * i] = '[';
                        expanded_grid[j][2 * i + 1] = ']';
                    }
                    _ => {
                        expanded_grid[j][2 * i] = '.';
                        expanded_grid[j][2 * i + 1] = '.';
                    }
                }
            }
        }

        let moves = moves
            .lines()
            .flat_map(|l| l.chars().map(Move::from))
            .collect();

        Self {
            grid: expanded_grid,
            moves,
            robot,
        }
    }

    fn from_str(input: &str) -> Self {
        let (grid, moves) = input.split_once("\n\n").unwrap();
        let grid: Vec<Vec<char>> = grid.lines().map(|l| l.chars().collect()).collect();

        let mut robot = (0, 0);
        for j in 0..grid.len() {
            for i in 0..grid[0].len() {
                if grid[j][i] == '@' {
                    robot = (i, j);
                }
            }
        }

        let moves = moves
            .lines()
            .flat_map(|l| l.chars().map(Move::from))
            .collect();

        Self { grid, moves, robot }
    }

    fn execute_moves(&mut self) {
        'outer: for m in &self.moves {
            match m {
                Move::Up => {
                    // Look up until we get a free space.
                    // Then just put a free space on the robot, and move everything else up by 1.
                    let mut next_free_y = 0;
                    for j in (0..=self.robot.1).rev() {
                        if self.grid[j][self.robot.0] == '#' {
                            continue 'outer;
                        }
                        if self.grid[j][self.robot.0] == '.' {
                            next_free_y = j;
                            break;
                        }
                    }
                    if next_free_y > 0 {
                        for j in next_free_y..self.robot.1 {
                            self.grid[j][self.robot.0] = self.grid[j + 1][self.robot.0];
                        }
                        self.grid[self.robot.1][self.robot.0] = '.';
                        self.robot = (self.robot.0, self.robot.1 - 1);
                    }
                }
                Move::Down => {
                    let mut next_free_y = self.grid.len();
                    for j in self.robot.1..self.grid.len() {
                        if self.grid[j][self.robot.0] == '#' {
                            continue 'outer;
                        }
                        if self.grid[j][self.robot.0] == '.' {
                            next_free_y = j;
                            break;
                        }
                    }
                    if next_free_y < self.grid.len() - 1 {
                        for j in (self.robot.1 + 1..=next_free_y).rev() {
                            self.grid[j][self.robot.0] = self.grid[j - 1][self.robot.0];
                        }
                        self.grid[self.robot.1][self.robot.0] = '.';
                        self.robot = (self.robot.0, self.robot.1 + 1);
                    }
                }
                Move::Left => {
                    let mut next_free_x = 0;
                    for i in (0..=self.robot.0).rev() {
                        if self.grid[self.robot.1][i] == '#' {
                            continue 'outer;
                        }
                        if self.grid[self.robot.1][i] == '.' {
                            next_free_x = i;
                            break;
                        }
                    }
                    if next_free_x > 0 {
                        for i in next_free_x..self.robot.0 {
                            self.grid[self.robot.1][i] = self.grid[self.robot.1][i + 1];
                        }
                        self.grid[self.robot.1][self.robot.0] = '.';
                        self.robot = (self.robot.0 - 1, self.robot.1);
                    }
                }
                Move::Right => {
                    let mut next_free_x = self.grid[0].len();
                    for i in self.robot.0..self.grid[0].len() {
                        if self.grid[self.robot.1][i] == '#' {
                            continue 'outer;
                        }
                        if self.grid[self.robot.1][i] == '.' {
                            next_free_x = i;
                            break;
                        }
                    }
                    if next_free_x < self.grid[0].len() - 1 {
                        for i in (self.robot.0 + 1..=next_free_x).rev() {
                            self.grid[self.robot.1][i] = self.grid[self.robot.1][i - 1];
                        }
                        self.grid[self.robot.1][self.robot.0] = '.';
                        self.robot = (self.robot.0 + 1, self.robot.1);
                    }
                }
            }
        }
    }

    fn gps(&self) -> usize {
        let mut score = 0;
        for j in 0..self.grid.len() {
            for i in 0..self.grid[0].len() {
                if self.grid[j][i] == 'O' {
                    score += 100 * j + i;
                }
            }
        }
        score
    }

    fn gps2(&self) -> usize {
        let mut score = 0;
        for j in 0..self.grid.len() {
            for i in 0..self.grid[0].len() {
                if self.grid[j][i] == '[' {
                    score += 100 * j + i;
                }
            }
        }
        score
    }

    fn execute_moves2(&mut self) {
        // Difference is, on the up and downs we have to keep track of the x indices that are the "blockers".
        // We also need to keep track of all the box positions.
        'outer: for m in &self.moves {
            match m {
                Move::Up => {
                    let mut box_positions = HashSet::<(usize, usize, usize, usize)>::new();
                    let mut current_blocking_indicies = HashSet::<usize>::new();

                    let mut current_y = self.robot.1;
                    current_blocking_indicies.insert(self.robot.0);
                    // Loop while so long as we are not blocked - if we ever hit a wall then we just go to the next move.
                    while !current_blocking_indicies.is_empty() {
                        let mut next_blocking_indices = HashSet::<usize>::new();
                        for i in current_blocking_indicies {
                            if self.grid[current_y - 1][i] == '[' {
                                next_blocking_indices.insert(i);
                                next_blocking_indices.insert(i + 1);
                                box_positions.insert((i, current_y - 1, i + 1, current_y - 1));
                            }
                            if self.grid[current_y - 1][i] == ']' {
                                next_blocking_indices.insert(i);
                                next_blocking_indices.insert(i - 1);
                                box_positions.insert((i - 1, current_y - 1, i, current_y - 1));
                            }
                            if self.grid[current_y - 1][i] == '#' {
                                continue 'outer;
                            }
                        }
                        // If we get here, then check if we are blocked - if we are not the we need to do a move.
                        current_blocking_indicies = next_blocking_indices;
                        current_y -= 1;
                    }

                    // delete the robot and the boxes, then redraw them moved up by 1.
                    for boxes in &box_positions {
                        self.grid[boxes.1][boxes.0] = '.';
                        self.grid[boxes.3][boxes.2] = '.';
                    }
                    self.grid[self.robot.1][self.robot.0] = '.';

                    for boxes in box_positions {
                        self.grid[boxes.1 - 1][boxes.0] = '[';
                        self.grid[boxes.3 - 1][boxes.2] = ']';
                    }
                    self.grid[self.robot.1 - 1][self.robot.0] = '@';
                    self.robot = (self.robot.0, self.robot.1 - 1);
                }
                Move::Down => {
                    let mut box_positions = HashSet::<(usize, usize, usize, usize)>::new();
                    let mut current_blocking_indicies = HashSet::<usize>::new();

                    let mut current_y = self.robot.1;
                    current_blocking_indicies.insert(self.robot.0);
                    // Loop while so long as we are not blocked - if we ever hit a wall then we just go to the next move.
                    while !current_blocking_indicies.is_empty() {
                        let mut next_blocking_indices = HashSet::<usize>::new();
                        for i in current_blocking_indicies {
                            if self.grid[current_y + 1][i] == '[' {
                                next_blocking_indices.insert(i);
                                next_blocking_indices.insert(i + 1);
                                box_positions.insert((i, current_y + 1, i + 1, current_y + 1));
                            }
                            if self.grid[current_y + 1][i] == ']' {
                                next_blocking_indices.insert(i);
                                next_blocking_indices.insert(i - 1);
                                box_positions.insert((i - 1, current_y + 1, i, current_y + 1));
                            }
                            if self.grid[current_y + 1][i] == '#' {
                                continue 'outer;
                            }
                        }
                        // If we get here, then check if we are blocked - if we are not the we need to do a move.
                        current_blocking_indicies = next_blocking_indices;
                        current_y += 1;
                    }

                    // delete the robot and the boxes, then redraw them moved down by 1.
                    for boxes in &box_positions {
                        self.grid[boxes.1][boxes.0] = '.';
                        self.grid[boxes.3][boxes.2] = '.';
                    }
                    self.grid[self.robot.1][self.robot.0] = '.';

                    for boxes in box_positions {
                        self.grid[boxes.1 + 1][boxes.0] = '[';
                        self.grid[boxes.3 + 1][boxes.2] = ']';
                    }
                    self.grid[self.robot.1 + 1][self.robot.0] = '@';
                    self.robot = (self.robot.0, self.robot.1 + 1);
                }
                // Left and right remain the same.
                Move::Left => {
                    let mut next_free_x = 0;
                    for i in (0..=self.robot.0).rev() {
                        if self.grid[self.robot.1][i] == '#' {
                            continue 'outer;
                        }
                        if self.grid[self.robot.1][i] == '.' {
                            next_free_x = i;
                            break;
                        }
                    }
                    if next_free_x > 0 {
                        for i in next_free_x..self.robot.0 {
                            self.grid[self.robot.1][i] = self.grid[self.robot.1][i + 1];
                        }
                        self.grid[self.robot.1][self.robot.0] = '.';
                        self.robot = (self.robot.0 - 1, self.robot.1);
                    }
                }
                Move::Right => {
                    let mut next_free_x = self.grid[0].len();
                    for i in self.robot.0..self.grid[0].len() {
                        if self.grid[self.robot.1][i] == '#' {
                            continue 'outer;
                        }
                        if self.grid[self.robot.1][i] == '.' {
                            next_free_x = i;
                            break;
                        }
                    }
                    if next_free_x < self.grid[0].len() - 1 {
                        for i in (self.robot.0 + 1..=next_free_x).rev() {
                            self.grid[self.robot.1][i] = self.grid[self.robot.1][i - 1];
                        }
                        self.grid[self.robot.1][self.robot.0] = '.';
                        self.robot = (self.robot.0 + 1, self.robot.1);
                    }
                }
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Move {
    fn from(value: char) -> Self {
        match value {
            '^' => Self::Up,
            '>' => Self::Right,
            'v' => Self::Down,
            '<' => Self::Left,
            _ => panic!("unexpected"),
        }
    }
}
