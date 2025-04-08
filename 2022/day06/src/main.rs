use std::fs;

fn main() {
    // let path = "input_example.txt";
    let path = "input.txt";

    let input: Vec<char> = fs::read_to_string(path).unwrap().chars().collect();

    for (i, win) in input.windows(4).enumerate() {
        if win
            .iter()
            .all(|c| win.iter().filter(|&wc| wc == c).count() == 1)
        {
            println!("Part one: {}", i + win.len());
            println!("window: {}", win.iter().collect::<String>());
            break;
        }
    }
    println!("--");

    for (i, win) in input.windows(14).enumerate() {
        if win
            .iter()
            .all(|c| win.iter().filter(|&wc| wc == c).count() == 1)
        {
            println!("Part two: {}", i + win.len());
            println!("window: {}", win.iter().collect::<String>());
            break;
        }
    }
}
