use anyhow::{Context, Result};

fn main() -> Result<()> {
    let path = "input.txt";
    // let path = "input_example.txt";

    let input = std::fs::read_to_string(path).context("Wrong file")?;

    let mut part_one = 0;
    let mut part_two = 0;
    for range in input.trim_end().split(',') {
        let (first, last) = range.split_once('-').context("Not a range")?;
        let first = first.parse::<u64>().context("First parsing fail.")?;
        let last = last.parse::<u64>().context("Last parsing fail.")?;

        for number in first..=last {
            part_one += is_invalid_p1(number);
            part_two += is_invalid_p2(number);
        }
    }
    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);

    Ok(())
}

fn is_invalid_p2(number: u64) -> u64 {
    let str: Vec<char> = number.to_string().chars().collect();

    for i in 1..=str.len() / 2 {
        if str.len().is_multiple_of(i) {
            let first = &str[..i];
            if str.windows(i).step_by(i).skip(1).all(|w| w == first) {
                return number;
            }
        }
    }
    0
}

fn is_invalid_p1(number: u64) -> u64 {
    let str = number.to_string();
    if str.len() > 2 && str.len().rem_euclid(2) == 0 {
        let (left, right) = str.split_at(str.len() / 2);
        if left == right {
            return number;
        }
    }
    0
}
