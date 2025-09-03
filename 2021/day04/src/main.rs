use std::collections::VecDeque;

const OFF: u32 = u32::MAX;

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let input = std::fs::read_to_string(path).unwrap();

    let numbers: Vec<u32> = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|txt| txt.parse::<u32>().unwrap())
        .collect();

    let mut boards: VecDeque<Vec<Vec<u32>>> = VecDeque::new();

    let mut current: Vec<Vec<u32>> = Vec::new();
    for (i, line) in input.lines().skip(2).filter(|l| !l.is_empty()).enumerate() {
        current.push(
            line.split_whitespace()
                .map(|txt| txt.parse::<u32>().unwrap())
                .collect(),
        );

        if (i + 1) % 5 == 0 {
            boards.push_back(current.clone());
            current.clear();
        }
    }

    // --
    let mut first_done = false;
    let mut to_remove = Vec::new();
    for number in numbers.iter() {
        boards.iter_mut().flatten().flatten().for_each(|n| {
            if n == number {
                *n = OFF
            }
        });

        // Are there boards done?
        for (i, board) in boards.iter().enumerate() {
            if is_done(board) {
                if !first_done {
                    first_done = true;
                    println!(
                        "Part one: {}",
                        number * board.iter().flatten().filter(|v| **v < OFF).sum::<u32>()
                    );
                } else if boards.len() == 1 {
                    println!(
                        "Part two: {}",
                        number * board.iter().flatten().filter(|v| **v < OFF).sum::<u32>()
                    );
                }

                to_remove.push(i);
            }
        }

        while let Some(index) = to_remove.pop() {
            boards.remove(index);
        }
    }
}

fn is_done(board: &[Vec<u32>]) -> bool {
    if board.iter().any(|line| line.iter().all(|v| *v == OFF)) {
        return true;
    }

    for i in 0..5 {
        if board.iter().all(|v| *v.get(i).unwrap() == OFF) {
            return true;
        }
    }

    false
}
