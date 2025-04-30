use std::fs;

const NEIGHBOURS: [(i16, i16); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn main() {
    // let path = "input_example.txt";
    let path = "input.txt";

    let input_base: Vec<Vec<char>> = fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    let mut input = input_base.clone();
    let mut updated = 1;
    while updated > 0 {
        (input, updated) = apply(input, adjacents_1, 4);
    }

    // display(&input);
    println!(
        "Part one: {}",
        input.iter().flatten().filter(|c| **c == '#').count()
    );

    // --
    let mut input = input_base.clone();
    let mut updated = 1;
    while updated > 0 {
        (input, updated) = apply(input, adjacents_2, 5);
    }

    // display(&input);
    println!(
        "Part two: {}",
        input.iter().flatten().filter(|c| **c == '#').count()
    );
}

fn apply(
    input: Vec<Vec<char>>,
    adj_fun: fn(i16, i16, &[Vec<char>]) -> Vec<char>,
    tolerance: usize,
) -> (Vec<Vec<char>>, u32) {
    let mut new = input.clone();
    let mut updated = 0;

    for row in 0..input.len() as i16 {
        for col in 0..input[0].len() as i16 {
            let adj = adj_fun(row, col, &input);
            let current = &mut new[row as usize][col as usize];

            if *current == 'L' && adj.iter().all(|a| *a != '#') {
                *current = '#';
                updated += 1;
            } else if *current == '#' && adj.iter().filter(|a| **a == '#').count() >= tolerance {
                *current = 'L';
                updated += 1;
            }
        }
    }
    (new, updated)
}

fn adjacents_1(row: i16, col: i16, input: &[Vec<char>]) -> Vec<char> {
    let mut ok = Vec::new();
    for neighbour in NEIGHBOURS.iter() {
        let r = row + neighbour.0;
        let c = col + neighbour.1;

        if r >= 0 && r < input.len() as i16 && c >= 0 && c < input[0].len() as i16 {
            ok.push(input[r as usize][c as usize]);
        }
    }
    ok
}
fn adjacents_2(row: i16, col: i16, input: &[Vec<char>]) -> Vec<char> {
    let mut ok = Vec::new();
    for neighbour in NEIGHBOURS.iter() {
        let mut r = row;
        let mut c = col;
        loop {
            r += neighbour.0;
            c += neighbour.1;

            if r >= 0 && r < input.len() as i16 && c >= 0 && c < input[0].len() as i16 {
                match input[r as usize][c as usize] {
                    '#' | 'L' => {
                        ok.push(input[r as usize][c as usize]);
                        break;
                    }
                    _ => {}
                }
            } else {
                break;
            }
        }
    }
    ok
}

#[allow(dead_code)]
fn display(input: &[Vec<char>]) {
    println!("{}", "-".repeat(20));

    for row in input.iter() {
        for col in row.iter() {
            print!("{}", col);
        }
        println!();
    }
}
