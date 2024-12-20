use std::{collections::HashMap, fs};

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let input = fs::read_to_string(path).unwrap();

    let mut patterns: Vec<String> = Vec::new();
    let mut desings: Vec<String> = Vec::new();

    for line in input.lines() {
        if patterns.is_empty() {
            for group in line.split(", ") {
                patterns.push(group.to_string());
            }
        } else if !line.is_empty() {
            desings.push(line.to_string());
        }
    }

    let mut cache = HashMap::new();
    let total = desings
        .iter()
        .filter(|d| is_desing_ok(d, &patterns, &mut cache) > 0)
        .count();

    dbg!(&cache);
    println!("Part one: {}", total);

    let total = desings.iter().fold(0, |acc, desing| {
        acc + is_desing_ok(desing, &patterns, &mut cache)
    });
    println!("Part two: {}", total);

    println!("cache size: {}", cache.len());
}

fn is_desing_ok(desing: &str, patterns: &Vec<String>, cache: &mut HashMap<String, usize>) -> usize {
    if let Some(value) = cache.get(desing) {
        return *value;
    }
    if desing.is_empty() {
        return 1;
    }

    let count = patterns
        .iter()
        .filter(|pattern| desing.starts_with(*pattern))
        .map(|pattern| is_desing_ok(&desing[pattern.len()..], patterns, cache))
        .sum();

    cache.insert(desing.to_string(), count);
    count
}
