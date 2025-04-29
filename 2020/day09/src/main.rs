use std::fs;

fn main() {
    // let path = "input_example.txt";
    // let preamble = 5;
    let path = "input.txt";
    let preamble = 25;

    let input = fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| l.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    for (i, twentysixth) in input.iter().enumerate().skip(preamble) {
        if let Some(values) = input.get(i.saturating_sub(preamble)..i) {
            if !values.iter().any(|&v| {
                values
                    .iter()
                    .filter(|v2| **v2 != v)
                    .any(|&v2| v + v2 == *twentysixth)
            }) {
                println!("Part one: {}", twentysixth);

                if let Some(values_from_start) = input.get(0..i) {
                    for (index, val) in values_from_start.iter().enumerate() {
                        let mut to_sum = vec![*val];
                        for nb in values_from_start.iter().skip(index + 1) {
                            to_sum.push(*nb);

                            match to_sum.iter().sum::<u64>().cmp(twentysixth) {
                                std::cmp::Ordering::Equal => {
                                    println!(
                                        "Part two: {}",
                                        to_sum.iter().min().unwrap() + to_sum.iter().max().unwrap()
                                    );
                                    break;
                                }
                                std::cmp::Ordering::Greater => continue,
                                _ => {}
                            }
                        }
                    }
                }

                break;
            }
        }
    }
}
