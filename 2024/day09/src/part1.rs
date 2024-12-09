use std::fs;

fn main() {
    // let path = "input.txt";
    let path = "input_example.txt";

    let input: Vec<u8> = fs::read(path).unwrap();
    let input: Vec<u64> = input
        .iter()
        .filter(|c| **c >= 48)
        .map(|c| *c as u64 - 48)
        .collect();

    // Part one --
    let mut spaced = Vec::new();
    let mut current_value = 0;
    for (i, c) in input.iter().enumerate() {
        for _ in 0..*c {
            if i % 2 == 0 {
                spaced.push(current_value);
            } else {
                spaced.push(u64::MAX);
            }
        }
        if i % 2 == 0 {
            current_value += 1;
        }
    }
    display(&spaced);

    let mut compacted = Vec::new();
    let mut values: Vec<u64> = spaced.iter().filter(|v| **v < u64::MAX).copied().collect();
    let lenght = values.len();

    for v in spaced.iter() {
        if *v == u64::MAX {
            compacted.push(values.pop().unwrap());
        } else {
            compacted.push(*v);
        }
        if compacted.len() == lenght {
            break;
        }
    }

    let checksum = compacted.iter().enumerate().fold(0, |acc, (i, v)| {
        if *v < u64::MAX {
            acc + (i as u64 * *v)
        } else {
            acc
        }
    });

    println!("Part one: {}", checksum);
}

fn display(input: &[u64]) {
    for c in input.iter() {
        if *c == u64::MAX {
            print!(".");
        } else {
            print!("{}", c);
        }
    }
    println!();
}
