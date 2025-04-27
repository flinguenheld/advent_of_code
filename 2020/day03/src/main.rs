use std::fs;

#[derive(Debug)]
struct Point {
    row: usize,
    col: usize,
}
impl Point {
    fn add_col(mut self, input: &[Vec<char>], nb: usize) -> Self {
        self.col += nb;
        self.col %= input[0].len();
        self
    }
    fn add_row(mut self, nb: usize) -> Self {
        self.row += nb;
        self
    }
}

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let input: Vec<Vec<char>> = fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    println!("Part one: {}", total(3, 1, &input));

    println!(
        "Part two: {}",
        [
            total(1, 1, &input),
            total(3, 1, &input),
            total(5, 1, &input),
            total(7, 1, &input),
            total(1, 2, &input),
        ]
        .iter()
        .product::<usize>()
    );
}

fn total(jump_col: usize, jump_row: usize, input: &[Vec<char>]) -> usize {
    let mut total = 0;
    let mut point = Point { row: 0, col: 0 };

    while point.row < input.len() {
        if input[point.row][point.col] == '#' {
            total += 1;
        }
        point = point.add_col(input, jump_col).add_row(jump_row);
    }

    total
}
