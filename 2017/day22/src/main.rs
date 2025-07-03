use std::collections::HashMap;

const DIRECTIONS: [Point; 4] = [
    Point { row: -1, col: 0 },
    Point { row: 0, col: 1 },
    Point { row: 1, col: 0 },
    Point { row: 0, col: -1 },
];

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Point {
    row: i32,
    col: i32,
}
impl Point {
    fn new(row: i32, col: i32) -> Self {
        Point { row, col }
    }

    fn next(mut self, direction: usize) -> Point {
        self.row += DIRECTIONS[direction].row;
        self.col += DIRECTIONS[direction].col;
        self
    }
}

#[derive(Clone)]
enum State {
    Weakened,
    Infected,
    Flagged,
}

fn main() {
    // let input = std::fs::read_to_string("input.txt").unwrap();
    let input = std::fs::read_to_string("input_example.txt").unwrap();

    let position_init = Point::new(
        input.lines().count() as i32 / 2,
        input.lines().next().unwrap().chars().count() as i32 / 2,
    );

    let mut start: HashMap<Point, State> = HashMap::new();
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.char_indices() {
            if c == '#' {
                start.insert(Point::new(row as i32, col as i32), State::Infected);
            }
        }
    }

    let mut grid = start.clone();
    let mut direction = 0_usize;
    let mut position = position_init;
    let mut burst_counter = 0;
    for _ in 0..10_000 {
        if let std::collections::hash_map::Entry::Vacant(g) = grid.entry(position) {
            direction = (direction as isize - 1).rem_euclid(DIRECTIONS.len() as isize) as usize;
            g.insert(State::Infected);
            burst_counter += 1;
        } else {
            direction = (direction + 1).rem_euclid(DIRECTIONS.len());
            grid.remove(&position);
        }

        position = position.next(direction);
    }

    println!("Part one: {}", burst_counter);

    // --
    let mut grid = start.clone();
    let mut direction = 0_usize;
    let mut position = position_init;
    let mut burst_counter = 0;
    for _ in 0..10_000_000 {
        if let Some(current) = grid.get_mut(&position) {
            match *current {
                State::Weakened => {
                    burst_counter += 1;
                    *current = State::Infected
                }
                State::Infected => {
                    direction = (direction + 1).rem_euclid(DIRECTIONS.len());

                    *current = State::Flagged
                }
                State::Flagged => {
                    direction = (direction + 2).rem_euclid(DIRECTIONS.len());

                    grid.remove(&position);
                }
            }
        } else {
            direction = (direction as isize - 1).rem_euclid(DIRECTIONS.len() as isize) as usize;
            grid.insert(position, State::Weakened);
        }

        position = position.next(direction);
    }

    println!("Part two: {}", burst_counter);
}
