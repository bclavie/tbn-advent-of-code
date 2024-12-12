use std::collections::{HashSet, VecDeque};

fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/inputs/12"));
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let shapes = get_shapes_from_grid(&grid);

    let part1 = shapes
        .iter()
        .fold(0, |acc, shape| acc + (shape.area() * shape.perimeter()));
    println!("part1: {}", part1);

    let part2 = shapes
        .iter()
        .fold(0, |acc, shape| acc + (shape.area() * shape.sides()));
    println!("part2: {}", part2);
}

fn get_shapes_from_grid(grid: &[Vec<char>]) -> Vec<Shape> {
    let mut shapes = vec![];
    let mut coloured = HashSet::<(usize, usize)>::new();

    for j in 0..grid.len() {
        for i in 0..grid[0].len() {
            if coloured.contains(&(i, j)) {
                continue;
            }

            let label = grid[j][i];
            let mut shape = Shape::new(label);
            let mut queue = VecDeque::new();
            queue.push_front((i, j));
            while let Some(coord) = queue.pop_back() {
                if coloured.contains(&coord) || grid[coord.1][coord.0] != label {
                    continue;
                }

                coloured.insert(coord);
                shape.coords.insert(coord);

                if coord.1 > 0 {
                    queue.push_back((coord.0, coord.1 - 1));
                }

                if coord.0 > 0 {
                    queue.push_back((coord.0 - 1, coord.1));
                }

                if coord.1 + 1 < grid.len() {
                    queue.push_back((coord.0, coord.1 + 1));
                }

                if coord.0 + 1 < grid[0].len() {
                    queue.push_back((coord.0 + 1, coord.1));
                }
            }

            shapes.push(shape);
        }
    }

    shapes
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
enum Edge {
    Top,
    Left,
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
enum EdgeInterior {
    Interior,
    Exterior,
}

#[derive(Clone)]
struct Shape {
    #[allow(unused)]
    label: char,
    coords: HashSet<(usize, usize)>,
}

impl Shape {
    fn new(label: char) -> Self {
        Self {
            label,
            coords: HashSet::new(),
        }
    }

    fn area(&self) -> usize {
        self.coords.len()
    }

    // Each coord has 4 possible "edges". Imagine this as a tuple of coord, and edge, we just count the edges that are
    // exterior to our shape.
    // To prevent double counting we always take the edge as described by "left" and "top" orientations
    // E.g. the bottom edge of (1,0) is the same as the top edge of (1,1). (Orientated downwards).
    fn perimeter(&self) -> usize {
        let mut edge_set = HashSet::<((usize, usize), Edge)>::new();

        for point in &self.coords {
            // Check up
            if point.1 == 0 || !self.coords.contains(&(point.0, point.1 - 1)) {
                edge_set.insert((*point, Edge::Top));
            }

            // Left
            if point.0 == 0 || !self.coords.contains(&(point.0 - 1, point.1)) {
                edge_set.insert((*point, Edge::Left));
            }

            // Down
            if !self.coords.contains(&(point.0, point.1 + 1)) {
                edge_set.insert(((point.0, point.1 + 1), Edge::Top));
            }

            // Right
            if !self.coords.contains(&(point.0 + 1, point.1)) {
                edge_set.insert(((point.0 + 1, point.1), Edge::Left));
            }
        }

        edge_set.len()
    }

    // Similar approach to perimeter, except we need to do another pass with the edges to
    // group edges together.
    // We also need to handle the "mobius" case - this can be done by adding another entry to our tuple to indicate
    // whether the edge is "interior" or "exterior" to our shape (when viewed as the top/left edge).
    fn sides(&self) -> usize {
        let mut unit_edge_set = HashSet::<((usize, usize), Edge, EdgeInterior)>::new();

        for point in &self.coords {
            // Check up
            if point.1 == 0 || !self.coords.contains(&(point.0, point.1 - 1)) {
                unit_edge_set.insert((*point, Edge::Top, EdgeInterior::Interior));
            }

            // Left
            if point.0 == 0 || !self.coords.contains(&(point.0 - 1, point.1)) {
                unit_edge_set.insert((*point, Edge::Left, EdgeInterior::Interior));
            }

            // Down
            if !self.coords.contains(&(point.0, point.1 + 1)) {
                unit_edge_set.insert(((point.0, point.1 + 1), Edge::Top, EdgeInterior::Exterior));
            }

            // Right
            if !self.coords.contains(&(point.0 + 1, point.1)) {
                unit_edge_set.insert(((point.0 + 1, point.1), Edge::Left, EdgeInterior::Exterior));
            }
        }

        // How do we group edges together? Basically do BFS again...
        let mut total_edges = 0;
        let mut done_unit_edges = HashSet::<((usize, usize), Edge, EdgeInterior)>::new();
        for unit_edge in &unit_edge_set {
            if done_unit_edges.contains(unit_edge) {
                continue;
            }

            let mut queue: VecDeque<((usize, usize), Edge, EdgeInterior)> = VecDeque::new();
            queue.push_front(*unit_edge);

            while let Some(ue) = queue.pop_back() {
                if done_unit_edges.contains(&ue) {
                    continue;
                }
                match ue.1 {
                    // Horiziontal
                    Edge::Top => {
                        if ue.0 .0 > 0 {
                            let left = ((ue.0 .0 - 1, ue.0 .1), ue.1, ue.2);
                            if unit_edge_set.contains(&left) {
                                queue.push_front(left);
                            }
                        }

                        let right = ((ue.0 .0 + 1, ue.0 .1), ue.1, ue.2);
                        if unit_edge_set.contains(&right) {
                            queue.push_front(right);
                        }
                    }
                    // Vertical
                    Edge::Left => {
                        if ue.0 .1 > 0 {
                            let up = ((ue.0 .0, ue.0 .1 - 1), ue.1, ue.2);
                            if unit_edge_set.contains(&up) {
                                queue.push_front(up);
                            }
                        }

                        let down = ((ue.0 .0, ue.0 .1 + 1), ue.1, ue.2);
                        if unit_edge_set.contains(&down) {
                            queue.push_front(down);
                        }
                    }
                }

                done_unit_edges.insert(ue);
            }

            total_edges += 1;
        }

        total_edges
    }
}
