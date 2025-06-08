use std::mem::swap;

struct Mask {
    mask: u128,
    rules: [u128; 4],
}

fn main() {
    let (path, nb_one, nb_two) = ("input.txt", 40, 400000);
    // let (path, nb_one, nb_two) = ("input_example.txt", 10, 10);
    // let (path, nb_one, nb_two) = ("input_example.txt", 3, 10);

    let input: Vec<char> = std::fs::read_to_string(path)
        .unwrap()
        .chars()
        .filter(|c| *c == '.' || *c == '^')
        .collect();

    println!("With vec of char: ");
    println!("Part one: {}", count(input.clone(), nb_one));
    println!("Part two: {}", count(input.clone(), nb_two));

    let mut safes = 0;
    let mut input: u128 = 0;
    let mut length = 0;
    for c in std::fs::read_to_string(path).unwrap().chars() {
        length += 1;
        match c {
            '.' => {
                input <<= 1;
                safes += 1
            }
            '^' => {
                input = input << 1 | 1;
            }
            _ => {}
        }
    }

    println!("\nWith bitwise operators: ");
    println!("Part one: {}", bit(input, nb_one, length, safes));
    println!("Part two: {}", bit(input, nb_two, length, safes));
}

fn bit(mut input: u128, number_of_rows: u32, row_length: u32, mut safes: usize) -> usize {
    // Get all masks
    let mut rules: [u128; 4] = [
        0b110 << (row_length - 2),
        0b011 << (row_length - 2),
        0b100 << (row_length - 2),
        0b001 << (row_length - 2),
    ];
    let mut mask: u128 = 0b111 << (row_length - 2);

    let mut masks: Vec<Mask> = Vec::new();
    while mask >= 0b111 {
        masks.push(Mask { mask, rules });

        rules.iter_mut().for_each(|r| *r >>= 1);
        mask >>= 1;
    }

    // Then build new row & count safe tiles
    // Shift the input to use same rules everywhere
    for _ in 1..number_of_rows {
        let mut new_row = 0;
        input <<= 1;
        for mask in masks.iter() {
            let current = input & mask.mask;

            new_row <<= 1;
            if mask.rules.iter().any(|&r| current == r) {
                new_row |= 1;
            } else {
                safes += 1;
            }
        }
        input = new_row;
    }
    safes
}

fn count(mut row: Vec<char>, number_of_rows: u32) -> usize {
    let mut total = row.iter().filter(|c| **c == '.').count();

    for _ in 0..number_of_rows - 1 {
        row.insert(0, '.');
        row.push('.');

        let mut next_row: Vec<char> = row
            .windows(3)
            .map(|win| match win {
                ['^', '^', '.'] | ['.', '^', '^'] | ['^', '.', '.'] | ['.', '.', '^'] => '^',
                _ => '.',
            })
            .collect();

        total += next_row.iter().filter(|c| **c == '.').count();
        swap(&mut row, &mut next_row);
    }
    total
}
