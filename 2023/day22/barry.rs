use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

fn main() {
    let inputs = include_str!("inputs/22");
    let mut grid = Grid::from(inputs);
    grid.update();

    let blocks_not_supporting_anything = grid
        .blocks
        .values()
        .filter(|b| b.supports.is_empty())
        .collect_vec();

    // Find all blocks that are supported by 2 blocks.
    // Get the unique names of the supporting blocks.
    let nominated_redundant_blocks = grid
        .blocks
        .values()
        .flat_map(|b| {
            let mut blocks = vec![];
            if b.supported_by.len() > 1 {
                for block in b.supported_by.iter() {
                    blocks.push(block.clone())
                }
            }
            blocks
        })
        .unique()
        .collect_vec();

    let mut redundant_blocks = HashSet::<String>::new();
    // For each of the supporting blocks - check if there are no blocks that
    // have this block as it's sole support.
    for b in nominated_redundant_blocks {
        let block = grid.blocks.get(&b).unwrap();
        let mut is_sole_support = false;
        for above in &block.supports {
            let above_block = grid.blocks.get(above).unwrap();
            if above_block.supported_by.len() == 1 {
                is_sole_support = true;
                break;
            }
        }

        if !is_sole_support {
            redundant_blocks.insert(b.clone());
        }
    }

    let part1 = blocks_not_supporting_anything.len() + redundant_blocks.len();
    println!("part1 {}", part1);

    // part2;
    let mut disintegrating_scores = HashMap::<String, u64>::new();
    for block in grid
        .blocks
        .values()
        .sorted_by(|a, b| a.bounds.0 .2.cmp(&b.bounds.0 .2))
    {
        disintegrating_scores.insert(
            block.name.to_string(),
            grid.disintegrating_scores(&block.name),
        );
    }
    let part2: u64 = disintegrating_scores.values().sum();
    println!("part2: {}", part2);
}

#[derive(Debug, Clone)]
struct Grid {
    // A few representation of blocks:
    // Map of block name -> block
    blocks: HashMap<String, Block>,
    // map to coordinate -> block name.
    occupied_coords: HashMap<Coord, String>,
}

impl Grid {
    // simulates the falling until things stop falling.
    fn update(&mut self) {
        self.update_supports();
        while self.simulate_falling() {
            self.simulate_falling();
            self.update_supports();
        }
    }

    // Returns whether or not any block has fallen or not.
    fn simulate_falling(&mut self) -> bool {
        let mut has_changed = false;
        // Always assumes the first bound has a lower Z value.
        let blocks_in_z_order = self
            .blocks
            .values_mut()
            .sorted_by(|a, b| a.bounds.0 .2.cmp(&b.bounds.0 .2))
            .collect_vec();
        for block in blocks_in_z_order {
            // 1 is the lowest position.
            if block.bounds.0 .2 == 1 {
                continue;
            }
            let coords_below = block.get_coords_below();
            let mut will_fall = true;
            for coord in coords_below {
                // If it's occupied by another, then ignore and continue;
                if self.occupied_coords.contains_key(&coord) {
                    will_fall = false;
                    break;
                }
            }

            if will_fall {
                has_changed = true;
                let current_coords = block.get_occupied_coords();
                for c in current_coords {
                    if self.occupied_coords.remove(&c).is_none() {
                        panic!("tried to remove coord not occupied")
                    }
                }

                block.drop_1();

                let new_coords = block.get_occupied_coords();
                for c in new_coords {
                    if self.occupied_coords.insert(c, block.name.clone()).is_some() {
                        panic!("tried to insert coord at occupied position");
                    }
                }
            }
        }

        has_changed
    }

    fn update_supports(&mut self) {
        for block in self.blocks.values_mut() {
            let coords_above = block.get_coords_above();
            let mut supported_blocks = HashSet::<String>::new();
            for c in coords_above {
                if let Some(block_name) = self.occupied_coords.get(&c) {
                    supported_blocks.insert(block_name.to_string());
                }
            }
            block.supports = supported_blocks;

            let coords_below = block.get_coords_below();
            let mut supporting_blocks = HashSet::<String>::new();
            for c in coords_below {
                if let Some(block_name) = self.occupied_coords.get(&c) {
                    supporting_blocks.insert(block_name.to_string());
                }
            }
            block.supported_by = supporting_blocks;
        }
    }

    fn disintegrating_scores(&self, block_name: &String) -> u64 {
        let block = self.blocks.get(block_name).unwrap();
        let mut blocks_to_fall = HashSet::<String>::new();
        blocks_to_fall.insert(block.name.clone());

        // BFS going upwards.
        let mut block_queue = VecDeque::<String>::new();

        for above in &block.supports {
            block_queue.push_back(above.clone());
        }

        while let Some(block_name) = block_queue.pop_front() {
            let current_block = self.blocks.get(&block_name).unwrap();

            let mut all_supports_fall = true;
            for b in &current_block.supported_by {
                if !blocks_to_fall.contains(b) {
                    all_supports_fall = false;
                    break;
                }
            }

            // If all our supports fell, then we fall too, and we need to add the blocks above us
            // to the queue.
            if all_supports_fall {
                blocks_to_fall.insert(block_name);
                for above in &current_block.supports {
                    block_queue.push_back(above.clone());
                }
            }
        }

        blocks_to_fall.len() as u64 - 1
    }
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let mut blocks = HashMap::new();
        let mut occupied_coords = HashMap::new();

        for (i, line) in input.lines().enumerate() {
            let name = i.to_string();
            let block = Block::from_input(name.clone(), line);
            let occupied = block.get_occupied_coords();

            for o in occupied {
                occupied_coords.insert(o, name.clone());
            }

            blocks.insert(name, block);
        }

        Self {
            blocks,
            occupied_coords,
        }
    }
}

#[derive(Debug, Clone)]
struct Block {
    name: String,
    bounds: (Coord, Coord),
    // The blocks that exist directly above this block (i.e. the ones that are supported by this block).
    supports: HashSet<String>,
    supported_by: HashSet<String>,
}

impl Block {
    #[allow(clippy::nonminimal_bool)]
    fn from_input(name: String, input: &str) -> Self {
        // 1,0,1~1,2,1
        let (bound1, bound2) = input.split_once('~').unwrap();
        let bounds = (coord_from_str(bound1), coord_from_str(bound2));

        // do a quick assertion that for each block we only differ on at most one axis.
        debug_assert!(
            bounds.0 .0 == bounds.1 .0 && bounds.0 .1 == bounds.1 .1
                || bounds.0 .0 == bounds.1 .0 && bounds.0 .2 == bounds.1 .2
                || bounds.0 .1 == bounds.1 .1 && bounds.0 .2 == bounds.1 .2
        );
        // Also assert that the bound's Z axis are lower or equal on the first bound
        debug_assert!(bounds.0 .2 <= bounds.1 .2);

        Self {
            name,
            bounds,
            supports: HashSet::<String>::new(),
            supported_by: HashSet::<String>::new(),
        }
    }

    fn get_occupied_coords(&self) -> Vec<Coord> {
        let mut coords = vec![];
        for x in self.bounds.0 .0..=self.bounds.1 .0 {
            for y in self.bounds.0 .1..=self.bounds.1 .1 {
                for z in self.bounds.0 .2..=self.bounds.1 .2 {
                    coords.push((x, y, z))
                }
            }
        }

        coords
    }

    fn drop_1(&mut self) {
        self.bounds.0 = (self.bounds.0 .0, self.bounds.0 .1, self.bounds.0 .2 - 1);
        self.bounds.1 = (self.bounds.1 .0, self.bounds.1 .1, self.bounds.1 .2 - 1);
    }

    // Gets the coords directly above this block
    fn get_coords_above(&self) -> Vec<Coord> {
        let my_coords = self.get_occupied_coords();
        let coords_above: Vec<Coord> = my_coords.iter().map(|c| (c.0, c.1, c.2 + 1)).collect_vec();
        let max_z = *coords_above.iter().max_by(|a, b| a.2.cmp(&b.2)).unwrap();

        coords_above
            .into_iter()
            .filter(|c| c.2 == max_z.2)
            .collect_vec()
    }

    fn get_coords_below(&self) -> Vec<Coord> {
        let my_coords = self.get_occupied_coords();
        let coords_below: Vec<Coord> = my_coords.iter().map(|c| (c.0, c.1, c.2 - 1)).collect_vec();
        let min_z = *coords_below.iter().min_by(|a, b| a.2.cmp(&b.2)).unwrap();

        coords_below
            .into_iter()
            .filter(|c| c.2 == min_z.2)
            .collect_vec()
    }
}

type Coord = (i64, i64, i64);

fn coord_from_str(s: &str) -> Coord {
    let [x, y, z] = s.splitn(3, ',').collect_vec()[0..3] else {
        panic!("unexpected pattern")
    };
    (x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap())
}
