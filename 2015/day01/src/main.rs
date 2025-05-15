use std::fs;

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";
    let input = fs::read_to_string(path).unwrap();

    println!(
        "Part one: {}",
        input.chars().fold(0, |acc, c| match c {
            '(' => acc + 1,
            ')' => acc - 1,
            _ => acc,
        })
    );

    let mut current = 0;
    for (i, c) in input.char_indices() {
        match c {
            '(' => current += 1,
            ')' => current -= 1,
            _ => {}
        }
        if current < 0 {
            println!("Part two: {}", i + 1);
            break;
        }
    }
}
