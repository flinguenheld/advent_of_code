use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let input = std::fs::read_to_string(path).unwrap();

    let mut people: HashSet<&str> = HashSet::new();
    let mut relations: HashMap<String, i32> = HashMap::new();

    for line in input.lines() {
        let mut iter = line.split_whitespace();

        let from = iter.next().unwrap();
        let operator = iter.nth(1).unwrap();
        let mut value = iter.next().unwrap().parse::<i32>().unwrap();
        if operator == "lose" {
            value *= -1;
        }
        let to: String = iter
            .last()
            .unwrap()
            .chars()
            .filter(|c| c.is_ascii_alphabetic())
            .collect();

        relations
            .entry(format!("{}{}", from, to))
            .and_modify(|v| *v += value)
            .or_insert(value);
        relations
            .entry(format!("{}{}", to, from))
            .and_modify(|v| *v += value)
            .or_insert(value);

        people.insert(from);
    }

    println!("Part one: {}", max(&people, &relations));

    // --
    for person in people.iter() {
        relations.insert(format!("Florent{}", person), 0);
        relations.insert(format!("{}Florent", person), 0);
    }
    people.insert("Florent");

    println!("Part two: {}", max(&people, &relations));
}

fn max(people: &HashSet<&str>, relations: &HashMap<String, i32>) -> i32 {
    let mut maxi = 0;

    for mut perm in people.iter().permutations(people.len()) {
        perm.push(perm.first().unwrap());
        maxi = std::cmp::max(
            maxi,
            perm.windows(2).fold(0, |acc, win| {
                acc + *relations.get(&format!("{}{}", win[0], win[1])).unwrap()
            }),
        );
    }
    maxi
}
