use std::collections::{BTreeSet, HashMap};

fn main() {
    let (path, nb_workers, base_time) = ("input.txt", 5, 60);
    // let (path, nb_workers, base_time) = ("input_example.txt", 2, 0);

    let mut input: HashMap<char, Vec<char>> = HashMap::new();
    for line in std::fs::read_to_string(path).unwrap().lines() {
        let mut it = line.split_whitespace();
        let must_be_done = it.nth(1).unwrap().chars().next().unwrap();
        let letter = it.nth(5).unwrap().chars().next().unwrap();

        input
            .entry(letter)
            .and_modify(|e| e.push(must_be_done))
            .or_insert(vec![must_be_done]);
    }

    // Add the first one
    let first = *input
        .values()
        .flatten()
        .find(|c| !input.contains_key(*c))
        .unwrap();
    input.insert(first, Vec::new());

    // --
    let mut rules = input.clone();
    let mut letters = Vec::new();
    let mut waiting: BTreeSet<char> = BTreeSet::new();

    while !rules.is_empty() {
        // Remove validated letters
        for (_, rule) in rules.iter_mut() {
            rule.retain(|v| !letters.contains(v));
        }

        // Get those which are availiable
        for (new_letter, _) in rules.iter().filter(|(_, v)| v.is_empty()) {
            waiting.insert(*new_letter);
        }

        // Add only the first one and remove it from rules
        if let Some(new_one) = waiting.pop_first() {
            rules.remove_entry(&new_one);
            letters.push(new_one);
        }
    }

    println!("Part one: {}", letters.iter().collect::<String>());

    // --
    let mut rules = input.clone();
    let mut workers: Vec<(u8, char)> = Vec::new();
    let mut letters = Vec::new();
    let mut time = 0;

    while !rules.is_empty() || !workers.is_empty() {
        time += 1;
        // Remove validated letters
        for (_, rule) in rules.iter_mut() {
            rule.retain(|v| !letters.contains(v));
        }

        // Update workers and fill letters when the work is done
        for (time, letter) in workers.iter_mut() {
            *time = time.saturating_sub(1);
            if *time == 0 {
                letters.push(*letter);
                rules.remove_entry(letter);
            }
        }

        // Free workers
        workers.retain(|(t, _)| *t > 0);

        // Get available letters
        for (new_letter, _) in rules.iter().filter(|(_, v)| v.is_empty()) {
            // And give them to workers
            if workers.len() < nb_workers
                && workers.iter().all(|(_, current)| current != new_letter)
            {
                workers.push((base_time + (*new_letter as u8 - 65), *new_letter));
            }
        }
    }

    println!("Part two: {}", time);
}
