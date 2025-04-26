use std::fs;

fn main() {
    // let path = "input_example.txt";
    let path = "input.txt";

    let input = fs::read_to_string(path).unwrap();

    let mut part_one = 0;
    let mut part_two = 0;

    for line in input.lines() {
        if let Some((policy, password)) = line.split_once(": ") {
            if let Some((range, letter)) = policy.split_once(' ') {
                if let Some((low, high)) = range.split_once('-') {
                    let low = low.parse::<usize>().unwrap_or(0);
                    let high = high.parse::<usize>().unwrap_or(low);

                    if let Some(letter) = letter.chars().next() {
                        let count = password.chars().filter(|c| *c == letter).count();

                        if count >= low && count <= high {
                            part_one += 1;
                        }

                        // Part two --
                        let first = password.chars().nth(low - 1).unwrap_or('0') == letter;
                        let second = password.chars().nth(high - 1).unwrap_or('0') == letter;

                        if (first && !second) || (!first && second) {
                            part_two += 1;
                        }
                    }
                }
            }
        }
    }

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);
}
