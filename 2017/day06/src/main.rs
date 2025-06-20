use std::collections::HashSet;

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let mut banks: Vec<u32> = std::fs::read_to_string(path)
        .unwrap()
        .split_whitespace()
        .map(|b| b.parse::<u32>().unwrap())
        .collect();

    let length = banks.len();
    let mut done: HashSet<Vec<u32>> = HashSet::new();
    let mut to_reach_part_two: Vec<u32> = Vec::new();
    let mut counter = 0;

    loop {
        let mut max = *banks.iter().max().unwrap();
        let mut index = banks.iter().position(|b| *b == max).unwrap();
        banks[index] = 0;

        while max > 0 {
            index += 1;
            max -= 1;
            banks[index.rem_euclid(length)] += 1;
        }

        counter += 1;

        if to_reach_part_two.is_empty() {
            if done.contains(&banks) {
                println!("Part one {}", counter);

                counter = 0;
                to_reach_part_two = banks.clone();
            } else {
                done.insert(banks.clone());
            }
        } else if banks == to_reach_part_two {
            println!("Part two {}", counter);
            break;
        }
    }
}
