use std::{
    collections::{HashMap, HashSet},
    fs,
};

struct Bag {
    nb: u32,
    name: String,
}

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let mut rules: HashMap<String, Vec<Bag>> = HashMap::new();

    for line in fs::read_to_string(path).unwrap().lines() {
        if let Some((left, right)) = line.split_once(" bags contain ") {
            let list: Vec<Bag> = if right.ends_with("no other bags.") {
                Vec::new()
            } else {
                right
                    .split(", ")
                    .map(|txt| {
                        let (nb, name) = txt.split_once(' ').unwrap();

                        Bag {
                            nb: nb.parse::<u32>().unwrap(),
                            name: name
                                .trim_end_matches(".")
                                .trim_end_matches(" bags")
                                .trim_end_matches(" bag")
                                .to_string(),
                        }
                    })
                    .collect()
            };

            rules.insert(left.to_string(), list);
        }
    }

    println!("Part one: {}", search("shiny gold", &rules).len());
    println!("Part two: {}", count("shiny gold", &rules) - 1); // Minus 1 for shiny gold itself
}

fn search(current: &str, rules: &HashMap<String, Vec<Bag>>) -> HashSet<String> {
    let mut ok = HashSet::new();
    for (key, values) in rules.iter() {
        if values.iter().any(|v| v.name == current) {
            ok.insert(key.clone());
            ok.extend(search(key, rules));
        }
    }
    ok
}

fn count(current: &str, rules: &HashMap<String, Vec<Bag>>) -> u32 {
    rules
        .get(current)
        .unwrap()
        .iter()
        .fold(1, |acc, bag| acc + bag.nb * count(&bag.name, rules))
}
