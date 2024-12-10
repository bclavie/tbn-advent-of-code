#[derive(PartialEq, Clone, Debug)]
enum State {
    FreeSpace,
    FileBlock,
}

impl State {
    fn flip(&mut self) -> Self {
        match self {
            State::FreeSpace => State::FileBlock,
            State::FileBlock => State::FreeSpace,
        }
    }
}

fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/inputs/9"));
    let mut total_length = 0;
    let mut state = State::FileBlock;
    let mut original_filesystem: Vec<FileSystem> = vec![];
    let mut current_id = 0;
    for c in input.chars() {
        let block_size = c.to_digit(10).unwrap() as u64;

        if block_size > 0 {
            original_filesystem.push(FileSystem::new(
                current_id, // ignored if space.
                total_length,
                total_length + block_size - 1,
                state.clone(),
            ));
        }

        if state == State::FileBlock {
            current_id += 1;
        }

        total_length += block_size;
        state = state.flip()
    }

    let mut filesystem_part2 = original_filesystem.clone();

    // for f in original_filesystem.clone() {
    //     println!("{:?}", f);
    // }

    // Have two scanners left and right.
    let mut left_scanner_idx = 0;
    let mut right_scanner_idx = total_length - 1;
    let mut left_file_block_idx = 0;
    let mut right_file_block_idx = original_filesystem.len() - 1;
    if original_filesystem[right_file_block_idx].state == State::FreeSpace {
        right_file_block_idx -= 1;
    }

    // Loop part1:
    let mut part1 = 0;
    loop {
        if left_scanner_idx > right_scanner_idx {
            break;
        }

        let current_block = &original_filesystem[left_file_block_idx];
        if current_block.state == State::FreeSpace {
            // Find find the next entry on the right to write this should always be a File.
            let right_block: &FileSystem = &original_filesystem[right_file_block_idx];
            debug_assert!(right_block.state == State::FileBlock);

            let score = left_scanner_idx * right_block.id;
            part1 += score;
            // print!("{}", score);

            // When we read from the right - move the right scanner back by 1 & check if we need to move into the next block
            // & update the scanner.
            right_scanner_idx -= 1;
            if !original_filesystem[right_file_block_idx].in_bounds(right_scanner_idx) {
                if right_file_block_idx == 0 {
                    break;
                }

                right_file_block_idx -= 1;
                while original_filesystem[right_file_block_idx].state == State::FreeSpace {
                    right_file_block_idx -= 1;
                }
                right_scanner_idx = original_filesystem[right_file_block_idx].end_idx;
            }
        } else {
            // Otherwise read the current space from the left.
            let score = left_scanner_idx * current_block.id;
            part1 += score;
            // print!("{}", score);
        }

        // Always increment the left scanner.
        left_scanner_idx += 1;
        if !original_filesystem[left_file_block_idx].in_bounds(left_scanner_idx) {
            left_file_block_idx += 1;
            left_scanner_idx = original_filesystem[left_file_block_idx].start_idx;
        }

        // print!("-");
    }
    // println!();

    // for f in original_filesystem {
    //     print!("{}", f);
    // }

    println!("part1: {}", part1);

    // Loop part2: Just manipulate the Filesystem structs directly, based on right pointer.
    let mut right_file_block_idx = filesystem_part2.len() - 1;
    while right_file_block_idx > 0 {
        if filesystem_part2[right_file_block_idx].state == State::FreeSpace
            || filesystem_part2[right_file_block_idx].has_swapped
        {
            right_file_block_idx -= 1;
            continue;
        }

        // We check exactly one per block on the right if we can find a spot that can move it.
        let mut found_block_to_swap = false;
        let mut left_idx_to_swap = 0;
        while left_idx_to_swap < right_file_block_idx {
            if filesystem_part2[left_idx_to_swap].state == State::FreeSpace
                && filesystem_part2[left_idx_to_swap].len()
                    >= filesystem_part2[right_file_block_idx].len()
            {
                found_block_to_swap = true;
                break;
            }
            left_idx_to_swap += 1;
        }

        if found_block_to_swap {
            let mut space: FileSystem = filesystem_part2[right_file_block_idx].clone();
            space.state = State::FreeSpace;

            filesystem_part2.insert(right_file_block_idx, space);
            let start_idx = filesystem_part2[left_idx_to_swap].start_idx;
            let mut right = filesystem_part2.remove(right_file_block_idx + 1);

            if right.len() == filesystem_part2[left_idx_to_swap].len() {
                filesystem_part2.remove(left_idx_to_swap);
                right_file_block_idx -= 1;
            } else {
                filesystem_part2[left_idx_to_swap].shrink(right.len());
            }
            right.reindex(start_idx);
            right.has_swapped = true;
            filesystem_part2.insert(left_idx_to_swap, right);
        } else {
            // If we haven't found a swap candidate, try the next one.
            right_file_block_idx -= 1;
        }
    }

    // for f in filesystem_part2.clone() {
    //     print!("{}", f);
    // }
    // println!();

    // Then calculate the score once all moves are done;
    let part2 = filesystem_part2.iter().fold(0, |acc, x| {
        if x.state == State::FileBlock {
            acc + x.score()
        } else {
            acc
        }
    });

    // 6424101383879 -> too high.
    println!("part2: {}", part2);
}

#[derive(Debug, Clone)]
struct FileSystem {
    id: u64,
    state: State,
    // Inclusive of Start
    start_idx: u64,
    // Inclusive of end.
    end_idx: u64,
    has_swapped: bool,
}

impl FileSystem {
    fn new(id: u64, start: u64, end: u64, state: State) -> Self {
        Self {
            id,
            state,
            start_idx: start,
            end_idx: end,
            has_swapped: false,
        }
    }
    fn in_bounds(&self, idx: u64) -> bool {
        idx >= self.start_idx && idx <= self.end_idx
    }
    fn len(&self) -> u64 {
        self.end_idx - self.start_idx + 1
    }
    fn shrink(&mut self, size: u64) {
        if size > self.len() {
            panic!("cannot shrink greater than current size");
        }
        self.start_idx += size;
    }
    fn score(&self) -> u64 {
        let mut score = 0;
        for i in self.start_idx..=self.end_idx {
            score += i * self.id
        }
        score
    }
    fn reindex(&mut self, new_start: u64) {
        debug_assert!(new_start < self.start_idx);
        self.end_idx = self.end_idx - self.start_idx + new_start;
        self.start_idx = new_start;
    }
}

impl std::fmt::Display for FileSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();

        match self.state {
            State::FreeSpace => {
                for _ in self.start_idx..=self.end_idx {
                    s.push('.');
                }
            }
            State::FileBlock => {
                for _ in self.start_idx..=self.end_idx {
                    s.push_str(&self.id.to_string());
                }
            }
        }
        f.write_str(&s)
    }
}
