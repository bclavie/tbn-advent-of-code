use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::{Display, Write},
};

fn main() {
    let inputs = include_str!("inputs/10");
    let mut pipe_grid = PipeGrid::new(inputs);

    let mut current_loc = pipe_grid.start;
    let mut current_step = 0;
    loop {
        pipe_grid.visited.insert(current_loc, current_step);
        match pipe_grid.get_next_tile(current_loc) {
            Some(next_loc) => {
                current_loc = next_loc;

                current_step += 1;
            }
            None => break,
        }
    }

    let part1 = pipe_grid.get_furthest_step_from_start();
    println!("part1: {}", part1);

    // part 2 -> The edge case of allowing squeezing through pipes screws with
    // the bfs colouring approach.
    // Unless we expand the resolution of the grid to 3x - keeping track of
    // which things in the new grid actually correspond to something on the old grid.
    //
    // - => becomes ...
    //              ---
    //              ...
    // J => becomes .|.
    //              -J.
    //              ...
    // etc..
    // We can actually simplify this further and just keep track of what tiles
    // in the new grid are covered by visited tiles.
    let mut expanded_grid = pipe_grid.expand_grid();
    expanded_grid.populate_locations_on_outside();

    let mut count = 0;
    for (y, lines) in expanded_grid.map.iter().enumerate() {
        for (x, _) in lines.iter().enumerate() {
            if !expanded_grid.is_visited((y, x))
                && !expanded_grid.locs_on_outside.contains(&(y, x))
                && expanded_grid.grid_mapping.contains_key(&(y, x))
            {
                count += 1;
            }
        }
    }
    println!("part2: {}", count);
}

struct PipeGrid {
    // Lines are the first one
    // chars are the 2nd, so a lookup is actually
    // [y][x]....
    map: Vec<Vec<Pipe>>,
    // Map from coord -> step from start
    visited: HashMap<(usize, usize), u64>,
    start: (usize, usize),
    locs_on_outside: HashSet<(usize, usize)>,
    // new -> old.
    grid_mapping: HashMap<(usize, usize), (usize, usize)>,
}

impl PipeGrid {
    #[allow(unused)]
    fn print(&self) {
        for l in self.map.iter() {
            println!();
            for p in l.iter() {
                print!("{}", p);
            }
        }
    }
    #[allow(unused)]
    fn print_colouring(&self) {
        let mut enclosed = 0;
        for (y, line) in self.map.iter().enumerate() {
            println!();
            for (x, p) in line.iter().enumerate() {
                if self.is_visited((y, x)) {
                    print!("\x1b[93m{}\x1b[0m", p);
                } else if self.locs_on_outside.contains(&(y, x)) {
                    print!("X")
                } else {
                    enclosed += 1;
                    print!("I")
                }
            }
        }
        println!();
        println!("Total enclosed: {}", enclosed);
    }

    fn new(input: &str) -> Self {
        let mut start_loc = (0_usize, 0_usize);

        let map: Vec<Vec<Pipe>> = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        let pipe = Pipe::from(c);
                        if pipe == Pipe::Start {
                            start_loc = (y, x);
                        }
                        pipe
                    })
                    .collect()
            })
            .collect();

        let visited = HashMap::new();
        let locs_on_outside = HashSet::<(usize, usize)>::new();
        let grid_mapping = HashMap::new();
        PipeGrid {
            map,
            visited,
            start: start_loc,
            locs_on_outside,
            grid_mapping,
        }
    }

    // Gets the next possible tile.
    // If we end up looping, then return None.
    fn get_next_tile(&self, current: (usize, usize)) -> Option<(usize, usize)> {
        match self.map[current.0][current.1] {
            Pipe::VertLine => {
                if self.is_visited((current.0 - 1, current.1))
                    && self.is_visited((current.0 + 1, current.1))
                {
                    None
                } else if !self.is_visited((current.0 - 1, current.1)) {
                    Some((current.0 - 1, current.1))
                } else {
                    Some((current.0 + 1, current.1))
                }
            }
            Pipe::HoriLine => {
                if self.is_visited((current.0, current.1 + 1))
                    && self.is_visited((current.0, current.1 - 1))
                {
                    None
                } else if !self.is_visited((current.0, current.1 - 1)) {
                    Some((current.0, current.1 - 1))
                } else {
                    Some((current.0, current.1 + 1))
                }
            }
            Pipe::L => {
                if self.is_visited((current.0 - 1, current.1))
                    && self.is_visited((current.0, current.1 + 1))
                {
                    None
                } else if !self.is_visited((current.0 - 1, current.1)) {
                    Some((current.0 - 1, current.1))
                } else {
                    Some((current.0, current.1 + 1))
                }
            }
            Pipe::J => {
                if self.is_visited((current.0 - 1, current.1))
                    && self.is_visited((current.0, current.1 - 1))
                {
                    None
                } else if !self.is_visited((current.0 - 1, current.1)) {
                    Some((current.0 - 1, current.1))
                } else {
                    Some((current.0, current.1 - 1))
                }
            }
            Pipe::Seven => {
                if self.is_visited((current.0 + 1, current.1))
                    && self.is_visited((current.0, current.1 - 1))
                {
                    None
                } else if !self.is_visited((current.0 + 1, current.1)) {
                    Some((current.0 + 1, current.1))
                } else {
                    Some((current.0, current.1 - 1))
                }
            }
            Pipe::F => {
                if self.is_visited((current.0 + 1, current.1))
                    && self.is_visited((current.0, current.1 + 1))
                {
                    None
                } else if !self.is_visited((current.0 + 1, current.1)) {
                    Some((current.0 + 1, current.1))
                } else {
                    Some((current.0, current.1 + 1))
                }
            }
            Pipe::Ground => {
                println!(
                    "what is at the current loc?: {}",
                    self.map[current.1][current.0]
                );
                panic!("unexpected ground location: {:?}", current)
            }
            Pipe::Start => {
                // Start is tricky since we don't know what it is;
                // Just look at the input and manually pick a direction!
                // Down looks like it satisfies both the sample input and my actual input.
                let next_loc = (current.0 + 1, current.1);
                Some(next_loc)
            }
        }
    }

    fn is_visited(&self, loc: (usize, usize)) -> bool {
        self.visited.contains_key(&loc)
    }

    fn get_furthest_step_from_start(&self) -> u64 {
        let (_, max_step) = self.visited.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap();

        (max_step + 1) / 2
    }

    fn expand_grid(&self) -> PipeGrid {
        let new_map: Vec<Vec<Pipe>> =
            vec![vec![Pipe::Ground; self.map[0].len() * 3]; self.map.len() * 3];
        let mut new_visited = HashMap::new();
        // We don't actually need to care about the start location for the expanded grid.
        let start = (0, 0);
        let locs_on_outside = HashSet::<(usize, usize)>::new();
        let mut grid_mapping = HashMap::new();

        for (y, lines) in self.map.iter().enumerate() {
            for (x, pipe) in lines.iter().enumerate() {
                // map the center of the new grid tile onto the old one.
                grid_mapping.insert((3 * y + 1, 3 * x + 1), (y, x));

                // If we aren't visited, then we can simplify and just leave it.
                if self.is_visited((y, x)) {
                    match pipe {
                        Pipe::VertLine => {
                            // .|.
                            // .|.
                            // .|.
                            // the actual step number doesn't matter in the expanded map.
                            new_visited.insert((y * 3, x * 3 + 1), 0);
                            new_visited.insert((y * 3 + 1, x * 3 + 1), 0);
                            new_visited.insert((y * 3 + 2, x * 3 + 1), 0);
                        }
                        Pipe::HoriLine => {
                            // ...
                            // ---
                            // ...

                            new_visited.insert((y * 3 + 1, x * 3), 0);
                            new_visited.insert((y * 3 + 1, x * 3 + 1), 0);
                            new_visited.insert((y * 3 + 1, x * 3 + 2), 0);
                        }
                        Pipe::L => {
                            // .|.
                            // .L-
                            // ...

                            new_visited.insert((y * 3, x * 3 + 1), 0);
                            new_visited.insert((y * 3 + 1, x * 3 + 1), 0);
                            new_visited.insert((y * 3 + 1, x * 3 + 2), 0);
                        }
                        Pipe::J => {
                            // .|.
                            // -J.
                            // ...

                            new_visited.insert((y * 3, x * 3 + 1), 0);
                            new_visited.insert((y * 3 + 1, x * 3), 0);
                            new_visited.insert((y * 3 + 1, x * 3 + 1), 0);
                        }
                        Pipe::Seven => {
                            // ...
                            // -7.
                            // .|.
                            new_visited.insert((y * 3 + 1, x * 3), 0);
                            new_visited.insert((y * 3 + 1, x * 3 + 1), 0);
                            new_visited.insert((y * 3 + 2, x * 3 + 1), 0);
                        }
                        Pipe::F => {
                            // ...
                            // .F-
                            // .|.
                            new_visited.insert((y * 3 + 1, x * 3 + 2), 0);
                            new_visited.insert((y * 3 + 1, x * 3 + 1), 0);
                            new_visited.insert((y * 3 + 2, x * 3 + 1), 0);
                        }
                        Pipe::Ground => {
                            panic!("unexpected visited ground")
                        }
                        Pipe::Start => {
                            // start is a special case. hard code this to match the input (ctrl c, ctrlv the correct match)
                            new_visited.insert((y * 3, x * 3 + 1), 0);
                            new_visited.insert((y * 3 + 1, x * 3 + 1), 0);
                            new_visited.insert((y * 3 + 2, x * 3 + 1), 0);
                        }
                    }
                }
            }
        }

        PipeGrid {
            map: new_map,
            visited: new_visited,
            start,
            locs_on_outside,
            grid_mapping,
        }
    }

    fn populate_locations_on_outside(&mut self) {
        // BFS queue.
        let mut visited_nodes_bfs = HashSet::<(usize, usize)>::new();
        let mut queue = VecDeque::new();
        for y in 0..self.map.len() {
            // If we are not in the loop, add to starting locs.
            if !self.is_visited((y, 0)) && !self.locs_on_outside.contains(&(y, 0)) {
                self.locs_on_outside.insert((y, 0));
                queue.push_back((y, 0));
            }
            if !self.is_visited((y, self.map[0].len() - 1))
                && !self.locs_on_outside.contains(&(y, self.map[0].len() - 1))
            {
                self.locs_on_outside.insert((y, self.map[0].len() - 1));
                queue.push_back((y, self.map[0].len() - 1))
            }
        }
        for x in 0..self.map[0].len() {
            // If we are not in the loop, add to starting locs.
            if !self.is_visited((0, x)) && !self.locs_on_outside.contains(&(0, x)) {
                self.locs_on_outside.insert((0, x));
                queue.push_back((0, x))
            }

            if !self.is_visited((self.map.len() - 1, x))
                && !self.locs_on_outside.contains(&(self.map.len() - 1, x))
            {
                self.locs_on_outside.insert((self.map.len() - 1, x));
                queue.push_back((self.map.len() - 1, x))
            }
        }

        // Start working on the queue.
        while let Some(loc) = queue.pop_front() {
            if loc.0 > self.map.len()
                || loc.1 > self.map[0].len()
                || visited_nodes_bfs.contains(&loc)
            {
                continue;
            }
            // Just need to check 4 cardinal directions if they are:
            // already coloured, or in the loop.
            self.locs_on_outside.insert(loc);

            // up
            if loc.0 > 0 {
                let loc_above = (loc.0 - 1, loc.1);
                if !self.is_visited(loc_above) && !self.locs_on_outside.contains(&loc_above) {
                    queue.push_back(loc_above);
                }
            }
            // Down
            if loc.0 < self.map.len() - 1 {
                let loc_below = (loc.0 + 1, loc.1);
                if !self.is_visited(loc_below) && !self.locs_on_outside.contains(&loc_below) {
                    queue.push_back(loc_below);
                }
            }

            // Left
            if loc.1 > 0 {
                let loc_left = (loc.0, loc.1 - 1);
                if !self.is_visited(loc_left) && !self.locs_on_outside.contains(&loc_left) {
                    queue.push_back(loc_left);
                }
            }

            // Right
            if loc.1 < self.map[0].len() - 1 {
                let loc_right = (loc.0, loc.1 + 1);
                if !self.is_visited(loc_right) && !self.locs_on_outside.contains(&loc_right) {
                    queue.push_back(loc_right);
                }
            }

            visited_nodes_bfs.insert(loc);
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Pipe {
    VertLine,
    HoriLine,
    L,
    J,
    Seven,
    F,
    Ground,
    Start,
}

impl From<char> for Pipe {
    fn from(value: char) -> Self {
        match value {
            '|' => Self::VertLine,
            '-' => Self::HoriLine,
            'L' => Self::L,
            'J' => Self::J,
            '7' => Self::Seven,
            'F' => Self::F,
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => panic!("unexpected char"),
        }
    }
}

impl Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pipe::VertLine => f.write_char('|'),
            Pipe::HoriLine => f.write_char('-'),
            Pipe::L => f.write_char('L'),
            Pipe::J => f.write_char('J'),
            Pipe::Seven => f.write_char('7'),
            Pipe::F => f.write_char('F'),
            Pipe::Ground => f.write_char('.'),
            Pipe::Start => f.write_char('S'),
        }
    }
}
