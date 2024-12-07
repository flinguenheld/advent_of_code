use colored::Colorize;
use std::fs;

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Point {
    row: usize,
    col: usize,
    max_row: i32,
    max_col: i32,
}

impl Point {
    fn new(row: usize, col: usize, max_row: usize, max_col: usize) -> Self {
        Point {
            row,
            col,
            max_row: max_row as i32,
            max_col: max_col as i32,
        }
    }

    fn next_pos(&self, direction_index: usize) -> Option<Point> {
        let new_row = self.row as i32 + DIRECTIONS[direction_index % 4].0;
        let new_col = self.col as i32 + DIRECTIONS[direction_index % 4].1;

        if new_row >= 0 && new_row < self.max_row && new_col >= 0 && new_col < self.max_col {
            Some(Point {
                row: new_row as usize,
                col: new_col as usize,
                ..*self
            })
        } else {
            None
        }
    }
}

fn main() {
    // let path = "input.txt";
    let path = "input_example.txt";

    let input = fs::read_to_string(path).unwrap();
    let mut input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    // Part one --
    let mut raaaa = 0;
    'aaa: for (r, row) in input.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            if *col == '^' {
                next(
                    Point::new(r, c, input.len(), input[0].len()),
                    &mut input,
                    0,
                    &mut raaaa,
                );
                break 'aaa;
            }
        }
    }

    display(&input);

    let count = input
        .iter()
        .flatten()
        .filter(|c| **c == 'X' || **c == '+' || **c == '^')
        .count();
    println!("Part one: {}", count);
    println!("Part two: {}", raaaa);
}

fn next(current_pos: Point, input: &mut [Vec<char>], mut direction_index: usize, raaa: &mut u32) {
    if input[current_pos.row][current_pos.col] == '.' {
        input[current_pos.row][current_pos.col] = 'X';
    }

    // Next case ?
    while let Some(next_position) = current_pos.next_pos(direction_index) {
        // Next to check if it can reach the current position
        if is_coming_back(current_pos, current_pos, input, direction_index + 1) {
            println!("true here {:?}", current_pos);
            *raaa += 1;
        }

        if input[next_position.row][next_position.col] == '#' {
            direction_index += 1;
            input[current_pos.row][current_pos.col] = '+';
        } else {
            next(next_position, input, direction_index, raaa);
            break;
        }
    }
}

fn is_coming_back(
    start: Point,
    current_pos: Point,
    input: &[Vec<char>],
    mut direction_index: usize,
) -> bool {
    while let Some(next_position) = current_pos.next_pos(direction_index) {
        let c = input[next_position.row][next_position.col];
        if c == '.' {
            // println!("NO => {}", c);

            // return false;
        }

        // Next to check if it can reach the current position
        if start == next_position {
            return true;
        }

        if input[next_position.row][next_position.col] == '#' {
            direction_index += 1;
        } else {
            return is_coming_back(start, next_position, input, direction_index);
        }
    }

    false
}

fn display(input: &[Vec<char>]) {
    for row in input.iter() {
        for col in row.iter() {
            match col {
                '.' => print!("{}", col.to_string().black()),
                // '.' => print!("{}", col.to_string().bright_black()),
                '#' => print!("{}", col.to_string().blue()),
                'X' => print!("{}", col.to_string().bright_black()),
                '+' => print!("{}", col.to_string().bright_red()),
                _ => print!("{}", col.to_string().red()),
            }
        }
        println!();
    }
}
