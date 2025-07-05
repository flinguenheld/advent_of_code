use std::collections::HashSet;

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    println!(
        "Part one: {}",
        std::fs::read_to_string(path)
            .unwrap()
            .lines()
            .fold(0, |acc, line| acc + line.parse::<i32>().unwrap())
    );

    // --
    let values: Vec<i32> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .collect();

    let mut done: HashSet<i32> = HashSet::new();
    let mut current = 0;
    for val in values.iter().cycle() {
        done.insert(current);

        current += val;
        if done.contains(&current) {
            println!("Part two: {}", current);
            break;
        }
    }
}
