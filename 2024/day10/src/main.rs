use std::fs;

#[derive(PartialEq)]
enum Dir {
    Start,
    North,
    East,
    Sth,
    West,
}

#[derive(PartialEq)]
struct Point {
    row: i32,
    col: i32,
}
impl Point {
    fn is_valid(&self, input: &[Vec<u32>]) -> bool {
        self.row >= 0
            && self.col >= 0
            && self.row < input.len() as i32
            && self.col < input[0].len() as i32
    }

    fn to(&self, direction: Dir) -> Self {
        match direction {
            Dir::North => Point {
                row: self.row - 1,
                col: self.col,
            },
            Dir::East => Point {
                row: self.row,
                col: self.col + 1,
            },
            Dir::Sth => Point {
                row: self.row + 1,
                col: self.col,
            },
            Dir::West => Point {
                row: self.row,
                col: self.col - 1,
            },
            Dir::Start => Point {
                row: self.row,
                col: self.col,
            },
        }
    }
}

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let input = fs::read_to_string(path).unwrap();
    let input: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| (c as u8 - 48) as u32).collect())
        .collect();

    let mut count = (0u32, 0u32);
    for r in 0..input.len() {
        for c in 0..input[r].len() {
            if input[r][c] == 0 {
                next(
                    &input,
                    Point {
                        row: r as i32,
                        col: c as i32,
                    },
                    0,
                    Dir::Start,
                    &mut count,
                    &mut Vec::new(),
                );
            }
        }
    }

    println!("Part one: {}", count.0);
    println!("Part two: {}", count.1);
}
fn next(
    input: &[Vec<u32>],
    start: Point,
    current_value: u32,
    from: Dir,
    count: &mut (u32, u32),
    found: &mut Vec<Point>,
) {
    if start.is_valid(input) {
        let new_val = input[start.row as usize][start.col as usize];
        if new_val == current_value + 1 || from == Dir::Start {
            if new_val == 9 {
                if !found.contains(&start) {
                    found.push(start);
                    count.0 += 1;
                }
                count.1 += 1;
            } else {
                if from != Dir::North {
                    next(input, start.to(Dir::North), new_val, Dir::Sth, count, found);
                }
                if from != Dir::East {
                    next(input, start.to(Dir::East), new_val, Dir::West, count, found);
                }
                if from != Dir::Sth {
                    next(input, start.to(Dir::Sth), new_val, Dir::North, count, found);
                }
                if from != Dir::West {
                    next(input, start.to(Dir::West), new_val, Dir::East, count, found);
                }
            }
        }
    }
}
