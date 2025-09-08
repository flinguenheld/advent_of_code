use std::collections::{BTreeSet, VecDeque};

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";
    // let path = "input_example_2.txt";

    let parenthesis = [('(', ')'), ('<', '>'), ('[', ']'), ('{', '}')];
    let mut count_p1 = 0;
    let mut count_p2 = BTreeSet::new();

    for line in std::fs::read_to_string(path).unwrap().lines() {
        let mut line: VecDeque<char> = line.chars().collect();

        'heps: loop {
            for i in 0..(line.len() - 1) {
                if parenthesis.contains(&(line[i], line[i + 1])) {
                    line.remove(i + 1);
                    line.remove(i);
                    continue 'heps;
                }
            }

            if line.is_empty() {
                break;
            } else if let Some(who) = line.iter().find(|c| ")]}>".contains(**c)) {
                count_p1 += match who {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    _ => 25137,
                };

                break;
            } else {
                // Incomplete --
                count_p2.insert(line.iter().rev().fold(0_u64, |acc, c| {
                    acc * 5
                        + match c {
                            '(' => 1,
                            '[' => 2,
                            '{' => 3,
                            _ => 4,
                        }
                }));
                break;
            }
        }
    }

    println!("Part one: {}", count_p1);
    println!(
        "Part two: {}",
        count_p2.iter().nth(count_p2.len() / 2).unwrap()
    );
}
