use std::fs;

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let input: Vec<String> = fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect();

    let mut total: u32 = 0;
    for line in input.iter() {
        let (left, right) = line.split_at(line.chars().count() / 2);

        if let Some(letter) = left.chars().filter(|&c| right.contains(c)).last() {
            total += letter_value(letter);
        }
    }
    println!("Part one: {}", total);

    let mut total: u32 = 0;
    for lines in input.windows(3).step_by(3) {
        if let Some(letter) = lines[0]
            .chars()
            .filter(|&c| lines[1].contains(c) && lines[2].contains(c))
            .last()
        {
            total += letter_value(letter);
        }
    }
    println!("Part two: {}", total);
}

fn letter_value(letter: char) -> u32 {
    match letter as u8 <= 91 {
        true => (letter as u8 + 26 - 64) as u32,
        false => (letter as u8 - 96) as u32,
    }
}
