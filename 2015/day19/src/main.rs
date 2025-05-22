use rand::Rng;
use std::collections::{HashMap, HashSet};

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";
    // let path = "input_example_2.txt";

    let input = std::fs::read_to_string(path).unwrap();

    let (rep, base) = input.split_once("\n\n").unwrap();
    let base: String = base.chars().filter(|c| c.is_ascii_alphabetic()).collect();

    let replacements: HashMap<&str, &str> = rep
        .lines()
        .map(|line| {
            let (from, to) = line.split_once(" => ").unwrap();
            (to, from)
        })
        .collect();

    let mut molecules: HashSet<String> = HashSet::new();
    for (&to, &from) in replacements.iter() {
        for (index, _) in base.match_indices(from) {
            molecules.insert(format!(
                "{}{}{}",
                &base[..index],
                to,
                &base[index + from.chars().count()..]
            ));
        }
    }

    println!("Part one: {}", molecules.len());

    // --
    let replacements: HashMap<&str, &str> = rep
        .lines()
        .map(|line| {
            let (from, to) = line.split_once(" => ").unwrap();
            (to, from)
        })
        .collect();

    // Ugly, there is obviously a better way -_-'
    // The order of the Hashmap impacts the result (which can never reach a 'e')
    // and test all possibilites is impossible.
    // So by taking patterns randomly it works and the minimum can be found by doing
    // that a lot of time.
    let mut rng = rand::rng();
    let mut min = u32::MAX;

    for _ in 0..100 {
        let mut current = base.clone();
        let mut total = 0;
        loop {
            total += 1;

            if current == *"e" {
                min = std::cmp::min(min, total);
                break;
            }

            if !replacements.iter().any(|(key, _)| current.contains(key)) {
                current = base.clone();
                total = 0;
            }

            while let Some((pat, to)) = replacements
                .iter()
                .nth(rng.random_range(..replacements.len()))
            {
                if let Some((left, right)) = current.split_once(pat) {
                    current = format!("{}{}{}", left, to, right);
                    break;
                }
            }
        }
    }

    println!("Part two: {}", min);
}
