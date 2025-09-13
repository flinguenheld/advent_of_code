use std::{collections::HashMap, mem};

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let template: Vec<char> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .next()
        .unwrap()
        .chars()
        .collect();

    let mut polymers: HashMap<[char; 2], u64> = HashMap::new();
    for win in template.windows(2) {
        polymers
            .entry([win[0], win[1]])
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }

    let rules: HashMap<[char; 2], char> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .skip(2)
        .map(|l| {
            let (left, right) = l.split_once(" -> ").unwrap();

            (
                [left.chars().next().unwrap(), left.chars().nth(1).unwrap()],
                right.chars().next().unwrap(),
            )
        })
        .collect();

    // Part one --
    let count = run_and_count(10, &template, &rules, polymers.clone());

    println!(
        "Part one: {}",
        count.iter().max_by_key(|(_, v)| **v).unwrap().1
            - count.iter().min_by_key(|(_, v)| **v).unwrap().1
    );

    // Part two --
    let count = run_and_count(40, &template, &rules, polymers);

    println!(
        "Part two: {}",
        count.iter().max_by_key(|(_, v)| **v).unwrap().1
            - count.iter().min_by_key(|(_, v)| **v).unwrap().1
    );
}

fn run_and_count(
    times: u8,
    template: &[char],
    rules: &HashMap<[char; 2], char>,
    mut polymer_test: HashMap<[char; 2], u64>,
) -> HashMap<char, u64> {
    for _ in 1..=times {
        let mut cache: HashMap<[char; 2], u64> = HashMap::new();
        for (pair, times) in polymer_test.iter() {
            if let Some(to_insert) = rules.get(pair) {
                cache
                    .entry([pair[0], *to_insert])
                    .and_modify(|e| *e += *times)
                    .or_insert(*times);
                cache
                    .entry([*to_insert, pair[1]])
                    .and_modify(|e| *e += *times)
                    .or_insert(*times);
            } else {
                cache.entry(*pair).and_modify(|e| *e += 1).or_insert(1);
            }
        }

        mem::swap(&mut polymer_test, &mut cache);
    }

    // Add the last template's letter which is not counted with this method
    let mut count: HashMap<char, u64> = HashMap::from([(*template.last().unwrap(), 1)]);
    for (who, times) in polymer_test.iter() {
        count
            .entry(who[0])
            .and_modify(|e| *e += times)
            .or_insert(*times);
    }

    count
}
