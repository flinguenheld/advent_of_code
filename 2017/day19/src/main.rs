use std::collections::HashMap;

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Hash, PartialEq, PartialOrd, Eq, Clone, Copy)]
struct Point {
    row: i16,
    col: i16,
}
impl Point {
    fn new(row: i16, col: i16) -> Self {
        Point { row, col }
    }

    fn next(mut self, direction: &Direction) -> Point {
        match direction {
            Direction::North => {
                self.row -= 1;
            }
            Direction::South => {
                self.row += 1;
            }
            Direction::East => {
                self.col += 1;
            }
            Direction::West => {
                self.col -= 1;
            }
        }
        self
    }
    fn neighbours(&self, exclude: &Direction) -> Vec<(Point, Direction)> {
        if *exclude == Direction::North || *exclude == Direction::South {
            vec![
                (self.next(&Direction::East), Direction::East),
                (self.next(&Direction::West), Direction::West),
            ]
        } else {
            vec![
                (self.next(&Direction::North), Direction::North),
                (self.next(&Direction::South), Direction::South),
            ]
        }
    }
}

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let mut grid: HashMap<Point, char> = HashMap::new();

    for (row, line) in std::fs::read_to_string(path).unwrap().lines().enumerate() {
        for (col, c) in line.char_indices().filter(|(_, c)| !c.is_whitespace()) {
            grid.insert(Point::new(row as i16, col as i16), c);
        }
    }

    // --
    let mut current = *grid
        .iter()
        .find(|(p, v)| p.row == 0 && **v == '|')
        .unwrap()
        .0;
    let mut direction = Direction::South;

    let mut text = String::new();
    let mut steps = 0;

    while let Some(value) = grid.get(&current) {
        if value.is_alphabetic() {
            text.push(*value);
        } else if *value == '+' {
            for (point, new_dir) in current.neighbours(&direction).iter() {
                if grid.contains_key(point) {
                    direction = *new_dir;
                }
            }
        }

        steps += 1;
        current = current.next(&direction);
    }

    println!("Part one: {}", text);
    println!("Part two: {}", steps);
}
