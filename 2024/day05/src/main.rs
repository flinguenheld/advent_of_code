use std::{collections::HashMap, fs};

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let input = fs::read_to_string(path).unwrap();

    let mut rules = HashMap::new();
    let mut all_lists: Vec<Vec<u16>> = Vec::new();
    let mut incorrect_lists: Vec<Vec<u16>> = Vec::new();

    for line in input.lines() {
        if let Some((left, right)) = line.split_once('|') {
            let left = left.parse::<u16>().unwrap();
            let right = right.parse::<u16>().unwrap();
            rules
                .entry(left)
                .and_modify(|e: &mut Vec<u16>| e.push(right))
                .or_insert(vec![right]);
        } else if !line.is_empty() {
            all_lists.push(
                line.split(',')
                    .map(|txt| txt.parse::<u16>().unwrap())
                    .collect(),
            );
        }
    }

    // Part one --
    let mut total = 0;
    for current_list in all_lists.iter() {
        match is_list_ok(current_list, &rules) {
            true => total += current_list.get(current_list.len() / 2).unwrap(),
            false => incorrect_lists.push(current_list.clone()),
        }
    }
    println!("Part one : {}", total);

    // Part two --
    let mut total = 0;
    for list in incorrect_lists.iter_mut() {
        let mut correction: Vec<u16> = Vec::new();

        while !list.is_empty() {
            let mut index = 0;
            for (i, value) in list.iter().enumerate() {
                if let Some(rule) = rules.get(value) {
                    if list.iter().all(|v| rule.contains(v) || v == value) {
                        index = i;
                        break;
                    }
                }
            }
            correction.push(list.remove(index));
        }
        total += correction.get(correction.len() / 2).unwrap();
    }
    println!("Part two : {}", total);
}

fn is_list_ok(list: &[u16], rules: &HashMap<u16, Vec<u16>>) -> bool {
    for (i, current_value) in list.iter().enumerate() {
        if let Some(rule) = rules.get(current_value) {
            if !list.iter().skip(i + 1).all(|v| rule.contains(v)) {
                return false;
            }
        } else if i != list.len() - 1 {
            return false;
        }
    }
    true
}
