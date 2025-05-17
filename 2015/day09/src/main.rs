use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn main() {
    // let path = "input_example.txt";
    let path = "input.txt";

    let mut to_do: HashSet<&str> = HashSet::new();
    let mut distances: HashMap<String, u32> = HashMap::new();
    let input = std::fs::read_to_string(path).unwrap();
    for line in input.lines() {
        let (places, distance) = line.split_once(" = ").unwrap();
        let (from, to) = places.split_once(" to ").unwrap();

        to_do.insert(from);
        to_do.insert(to);

        let distance = distance.parse::<u32>().unwrap();

        distances.insert(format!("{}{}", from, to), distance);
        distances.insert(format!("{}{}", to, from), distance);
    }

    let all = to_do
        .iter()
        .permutations(to_do.len())
        .map(|permutation| {
            permutation.windows(2).fold(0, |acc, win| {
                acc + distances.get(&format!("{}{}", win[0], win[1])).unwrap()
            })
        })
        .collect::<Vec<u32>>();

    println!("Part one: {}", all.iter().min().unwrap());
    println!("Part two: {}", all.iter().max().unwrap());
}
