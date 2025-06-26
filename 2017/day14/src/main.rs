use std::collections::VecDeque;

struct Point {
    row: usize,
    col: usize,
}
impl Point {
    fn new(row: usize, col: usize) -> Self {
        Point { row, col }
    }
    fn neighbours(&self, grid: &[Vec<char>]) -> Vec<Point> {
        let mut n = Vec::new();

        if self.row > 0 {
            n.push(Point::new(self.row - 1, self.col));
        }
        if self.row < grid.len() - 1 {
            n.push(Point::new(self.row + 1, self.col));
        }
        if self.col > 0 {
            n.push(Point::new(self.row, self.col - 1));
        }
        if self.col < grid[0].len() - 1 {
            n.push(Point::new(self.row, self.col + 1));
        }
        n
    }
}

fn main() {
    // let input = "AoC 2017";
    // let input = "a0c2017";
    // let input = "flqrgnkx";
    let input = "wenycdww";

    let mut grid = Vec::new();
    for i in 0..128 {
        let mut hash = knot_hash(format!("{}-{}", input, i).as_str());

        let mut row: Vec<char> = Vec::new();
        while let Some(mut h) = hash.pop_front() {
            let mut new_8_bits = VecDeque::new();
            while h > 0 {
                match h & 1 {
                    1 => new_8_bits.push_front('#'),
                    _ => new_8_bits.push_front('.'),
                }
                h >>= 1;
            }
            // Do no forget zeros
            while new_8_bits.len() < 8 {
                new_8_bits.push_front('.');
            }

            row.extend(new_8_bits.iter());
        }
        grid.push(row);
    }

    println!(
        "Part one: {}",
        grid.iter().flatten().filter(|c| **c == '#').count()
    );

    let mut counter = 0;
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == '#' {
                counter += 1;
                eat(Point::new(row, col), &mut grid);
            }
        }
    }
    println!("Part two: {}", counter);

    print(&grid);
}

// Convert contiguous '#' into 'O'
fn eat(current: Point, grid: &mut Vec<Vec<char>>) {
    grid[current.row][current.col] = 'O';
    for neighbour in current.neighbours(grid) {
        if grid[neighbour.row][neighbour.col] == '#' {
            eat(neighbour, grid);
        }
    }
}

// This hash consists of:
//    - Take a list 0..255
//    - Loop 64 times sparse_hash_round and keep the index and the skip value
//    - convert all chars to num of value
//    - Add "17, 31, 73, 47, 23" to the end
//    - Combine value and the sparse hash with the bitwise XOR
fn knot_hash(value: &str) -> VecDeque<u8> {
    let mut value = value.chars().map(|v| v as u8).collect::<Vec<u8>>();
    value.extend([17, 31, 73, 47, 23].iter());

    let mut sparse_hash: Vec<u8> = (0..=255).collect();
    let mut current_index = 0;
    let mut skip = 0;
    for _ in 0..64 {
        sparse_hash_round(&value, &mut sparse_hash, &mut current_index, &mut skip);
    }

    sparse_hash
        .windows(16)
        .step_by(16)
        .map(|win| win.iter().fold(0, |acc, v| acc ^ v))
        .collect()
    // sparse_hash
    //     .windows(16)
    //     .step_by(16)
    //     .fold(String::new(), |mut hash, win| {
    //         write!(hash, "{:08b}", win.iter().fold(0, |acc, v| acc ^ v)).ok();
    //         hash
    //     })
}

// Apply the https://adventofcode.com/2017/day/10 hash logic
fn sparse_hash_round(
    values: &[u8],
    sparse_hash: &mut [u8],
    curr_index: &mut usize,
    skip_size: &mut usize,
) {
    let list_length = sparse_hash.len();
    for length in values.iter() {
        let temp = sparse_hash.to_owned();

        let mut rev = *curr_index as i32 + *length as i32 - 1;
        for _ in 0..*length {
            sparse_hash[curr_index.rem_euclid(list_length)] =
                temp[rev.rem_euclid(list_length as i32) as usize];

            *curr_index += 1;
            rev -= 1;
        }

        *curr_index += *skip_size;
        *skip_size += 1;
    }
}

#[allow(dead_code)]
fn print(grid: &[Vec<char>]) {
    for line in grid.iter() {
        for c in line.iter() {
            print!("{}", c);
        }
        println!();
    }
}
