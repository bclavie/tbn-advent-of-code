fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/inputs/4"));

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut part1 = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            // Horizontal Left
            if y > 2
                && grid[y][x] == 'X'
                && grid[y - 1][x] == 'M'
                && grid[y - 2][x] == 'A'
                && grid[y - 3][x] == 'S'
            {
                part1 += 1;
            }

            // Horizontal Right
            if (y + 3) < grid.len()
                && grid[y][x] == 'X'
                && grid[y + 1][x] == 'M'
                && grid[y + 2][x] == 'A'
                && grid[y + 3][x] == 'S'
            {
                part1 += 1;
            }

            // Vertical Up
            if x > 2
                && grid[y][x] == 'X'
                && grid[y][x - 1] == 'M'
                && grid[y][x - 2] == 'A'
                && grid[y][x - 3] == 'S'
            {
                part1 += 1;
            }

            // Vertical Down
            if (x + 3) < grid[0].len()
                && grid[y][x] == 'X'
                && grid[y][x + 1] == 'M'
                && grid[y][x + 2] == 'A'
                && grid[y][x + 3] == 'S'
            {
                part1 += 1;
            }

            // Diagonals
            // Up Left
            if x > 2
                && y > 2
                && grid[y][x] == 'X'
                && grid[y - 1][x - 1] == 'M'
                && grid[y - 2][x - 2] == 'A'
                && grid[y - 3][x - 3] == 'S'
            {
                part1 += 1;
            }

            // Up Right
            if (x + 3) < grid[0].len()
                && y > 2
                && grid[y][x] == 'X'
                && grid[y - 1][x + 1] == 'M'
                && grid[y - 2][x + 2] == 'A'
                && grid[y - 3][x + 3] == 'S'
            {
                part1 += 1;
            }

            // Down Left
            if x > 2
                && (y + 3) < grid.len()
                && grid[y][x] == 'X'
                && grid[y + 1][x - 1] == 'M'
                && grid[y + 2][x - 2] == 'A'
                && grid[y + 3][x - 3] == 'S'
            {
                part1 += 1;
            }

            // Down Right
            if (x + 3) < grid[0].len()
                && (y + 3) < grid.len()
                && grid[y][x] == 'X'
                && grid[y + 1][x + 1] == 'M'
                && grid[y + 2][x + 2] == 'A'
                && grid[y + 3][x + 3] == 'S'
            {
                part1 += 1;
            }
        }
    }

    println!("part1: {}", part1);

    let mut part2 = 0;

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            // Do logic based on the Center A
            if x > 0
                && y > 0
                && (x + 1) < grid[0].len()
                && (y + 1) < grid.len()
                && grid[y][x] == 'A'
            {
                // M S
                //  A
                // M S
                if grid[y - 1][x - 1] == 'M'
                    && grid[y + 1][x - 1] == 'M'
                    && grid[y - 1][x + 1] == 'S'
                    && grid[y + 1][x + 1] == 'S'
                {
                    part2 += 1;
                }

                // M M
                //  A
                // S S
                if grid[y - 1][x - 1] == 'M'
                    && grid[y + 1][x - 1] == 'S'
                    && grid[y - 1][x + 1] == 'M'
                    && grid[y + 1][x + 1] == 'S'
                {
                    part2 += 1;
                }

                // S M
                //  A
                // S M
                if grid[y - 1][x - 1] == 'S'
                    && grid[y + 1][x - 1] == 'S'
                    && grid[y - 1][x + 1] == 'M'
                    && grid[y + 1][x + 1] == 'M'
                {
                    part2 += 1;
                }

                // S S
                //  A
                // M M
                if grid[y - 1][x - 1] == 'S'
                    && grid[y + 1][x - 1] == 'M'
                    && grid[y - 1][x + 1] == 'S'
                    && grid[y + 1][x + 1] == 'M'
                {
                    part2 += 1;
                }
            }
        }
    }

    println!("part2: {}", part2);
}
