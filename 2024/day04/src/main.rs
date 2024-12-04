use std::fs;

const DIRECTIONS: [(i32, i32); 8] = [
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
];

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";
    // let path = "input_example_2.txt";

    let input = fs::read_to_string(path).unwrap();
    let input: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    // Part one --
    let mut total = 0;
    for (row, line) in input.iter().enumerate() {
        for col in 0..line.len() {
            for dir in DIRECTIONS.iter() {
                next(row as i32, col as i32, '0', *dir, &input, &mut total);
            }
        }
    }

    println!("Part one: {}", total);

    // Part two --
    let mut total = 0;
    for (row, line) in input.iter().enumerate() {
        for (col, c) in line.iter().enumerate() {
            if *c == 'A' && row > 0 && col > 0 && row < input.len() - 1 && col < line.len() - 1 {
                let a = input[row - 1][col + 1];
                let b = input[row + 1][col - 1];

                let c = input[row - 1][col - 1];
                let d = input[row + 1][col + 1];

                if ((a == 'M' && b == 'S') || (a == 'S' && b == 'M'))
                    && ((c == 'M' && d == 'S') || (c == 'S' && d == 'M'))
                {
                    total += 1;
                }
            }
        }
    }

    println!("Part two: {}", total);
}

fn next(
    row: i32,
    col: i32,
    previous: char,
    direction: (i32, i32),
    input: &Vec<Vec<char>>,
    total: &mut u32,
) {
    let current_letter = input[row as usize][col as usize];

    match (previous, current_letter) {
        ('0', 'X') | ('X', 'M') | ('M', 'A') => {
            let next_row = row + direction.0;
            let next_col = col + direction.1;

            if next_row >= 0
                && next_row < input.len() as i32
                && next_col >= 0
                && next_col < input[0].len() as i32
            {
                next(next_row, next_col, current_letter, direction, input, total);
            }
        }
        ('A', 'S') => {
            *total += 1;
        }
        _ => {}
    }
}
