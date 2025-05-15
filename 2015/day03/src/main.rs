use std::{collections::HashSet, fs};

#[derive(Hash, PartialEq, PartialOrd, Eq, Clone, Copy, Debug)]
struct Point {
    row: i32,
    col: i32,
}
#[rustfmt::skip]
impl Point {
    fn go(&self, direction: char) -> Point {
        match direction {
            '>' => Point { row: self.row + 1, ..*self },
            '<' => Point { row: self.row - 1, ..*self },
            '^' => Point { col: self.col + 1, ..*self },
            _   => Point { col: self.col - 1, ..*self },
        }
    }
}

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let input = fs::read_to_string(path).unwrap();

    let mut santa = Point { row: 0, col: 0 };
    let mut visited: HashSet<Point> = HashSet::from([santa]);

    for c in input.chars().filter(|c| ['>', '<', '^', 'v'].contains(c)) {
        santa = santa.go(c);
        visited.insert(santa);
    }

    println!("Part one: {}", visited.len());

    // --
    let mut santa = Point { row: 0, col: 0 };
    let mut robot_santa = Point { row: 0, col: 0 };
    let mut visited: HashSet<Point> = HashSet::from([santa]);

    for (i, c) in input
        .char_indices()
        .filter(|(_, c)| ['>', '<', '^', 'v'].contains(c))
    {
        if i % 2 == 0 {
            santa = santa.go(c);
            visited.insert(santa);
        } else {
            robot_santa = robot_santa.go(c);
            visited.insert(robot_santa);
        }
    }

    println!("Part two: {}", visited.len());
}
