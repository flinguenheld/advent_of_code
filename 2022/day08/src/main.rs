use std::{cmp, fs};

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let input: Vec<Vec<u8>> = fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| l.chars().map(|c| c as u8 - 48).collect())
        .collect();

    let total = (0..input.len()).fold(0, |acc, row| {
        acc + (0..input[0].len()).fold(0, |acc2, col| match is_visible(row, col, &input) {
            true => acc2 + 1,
            false => acc2,
        })
    });

    println!("Part one: {}", total);

    let mut scenic_max = 0;
    for row in 0..input.len() {
        for col in 0..input[0].len() {
            scenic_max = cmp::max(scenic_score(row, col, &input), scenic_max);
        }
    }
    println!("Part two: {}", scenic_max);
}

fn is_visible(row: usize, col: usize, input: &[Vec<u8>]) -> bool {
    row == 0
        || row >= input.len()
        || col == 0
        || col >= input[0].len()
        || input.iter().take(row).all(|r| r[col] < input[row][col])
        || input.iter().skip(row + 1).all(|r| r[col] < input[row][col])
        || input[row].iter().take(col).all(|&c| c < input[row][col])
        || input[row]
            .iter()
            .skip(col + 1)
            .all(|&c| c < input[row][col])
}

fn scenic_score(row: usize, col: usize, input: &[Vec<u8>]) -> u32 {
    let mut top = 0;
    for r in input.iter().take(row).rev() {
        top += 1;
        if r[col] >= input[row][col] {
            break;
        }
    }

    let mut bottom = 0;
    for r in input.iter().skip(row + 1) {
        bottom += 1;
        if r[col] >= input[row][col] {
            break;
        }
    }

    let mut left = 0;
    for c in input[row].iter().take(col).rev() {
        left += 1;
        if *c >= input[row][col] {
            break;
        }
    }
    let mut right = 0;
    for c in input[row].iter().skip(col + 1) {
        right += 1;
        if *c >= input[row][col] {
            break;
        }
    }

    top * bottom * left * right
}
