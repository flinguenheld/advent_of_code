use std::fs;

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let mut input: Vec<u32> = fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(to_int)
        .collect();

    input.sort();
    println!("Part one: {}", input.last().unwrap());

    for win in input.windows(2) {
        if win[1] != win[0] + 1 {
            println!("Part two: {}", win[0] + 1);
            break;
        }
    }
}

fn to_int(code: &str) -> u32 {
    let mut value = 0;
    for (i, c) in code.chars().rev().enumerate() {
        if c == 'B' || c == 'R' {
            value |= 1 << i;
        }
    }

    value
}
