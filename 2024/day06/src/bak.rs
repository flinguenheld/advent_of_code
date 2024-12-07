use colored::Colorize;
use std::fs;

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

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
    'aaa: for (r, row) in input.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            if *col == '^' {
                next(Point::new(r, c, input.len(), input[0].len()), &mut input, 0);
                break 'aaa;
            }
        }
    }

    display(&input);

    let count = input.iter().flatten().filter(|c| **c == 'X').count();
    println!("Part one: {}", count);
}

fn next(current_pos: Point, input: &mut [Vec<char>], mut direction_index: usize) {
    if input[current_pos.row][current_pos.col] == '.' {
        input[current_pos.row][current_pos.col] = 'X';
    }

    // Next case ?
    while let Some(next_position) = current_pos.next_pos(direction_index) {
        if input[next_position.row][next_position.col] == '#' {
            direction_index += 1;
            input[current_pos.row][current_pos.col] = '+';
        } else {
            next(next_position, input, direction_index);
            break;
        }
    }
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
