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
    // println!("Hello, world!");

    // let path = "input.txt";
    let path = "input_example.txt";

    let input = fs::read_to_string(path).unwrap();
    let input: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut total = 0;
    for (row, line) in input.iter().enumerate() {
        for (col, c) in line.iter().enumerate() {
            // if *c == 'X' {
            next(row as i32, col as i32, '0', &input, &mut total);
            // }
        }
    }

    display(&input);
    println!("Part one: {}", total);
}

fn next(row: i32, col: i32, previous: char, input: &Vec<Vec<char>>, total: &mut u32) {
    // Launch next in all direction
    let current_letter = input[row as usize][col as usize];
    dbg!(current_letter);

    // if let Some(last_letter) = letters.last() {
    match (previous, current_letter) {
        ('0', 'X') | ('X', 'M') | ('M', 'A') => {
            for (dir_row, dir_col) in DIRECTIONS.iter() {
                let next_row = row + dir_row;
                let next_col = col + dir_col;

                if next_row >= 0
                    && next_row < input.len() as i32
                    && next_col >= 0
                    && next_col < input[0].len() as i32
                {
                    next(next_row, next_col, current_letter, input, total);
                }
            }
        }
        ('A', 'S') => {
            // dbg!(&letters);
            *total += 1;
        }
        _ => {}
    }
}

fn display(input: &Vec<Vec<char>>) {
    for line in input.iter() {
        print!("   ");
        for c in line.iter() {
            print!("{}", c);
        }
        println!();
    }
}
