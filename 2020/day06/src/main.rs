use std::{collections::HashSet, fs};

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let input: Vec<HashSet<char>> = fs::read_to_string(path)
        .unwrap()
        .split("\n\n")
        .map(|txt| txt.chars().filter(|c| c.is_ascii_alphabetic()).collect())
        .collect();

    println!("Part one: {}", input.iter().fold(0, |acc, l| acc + l.len()));

    // --
    let input = fs::read_to_string(path).unwrap();
    let input: Vec<Vec<&str>> = input
        .split("\n\n")
        .map(|group| group.lines().collect())
        .collect();

    let total = input.iter().fold(0, |acc, group| {
        acc + group[0]
            .chars()
            .filter(|&c| group.iter().all(|g| g.contains(c)))
            .count()
    });
    println!("Part two: {}", total);
}
