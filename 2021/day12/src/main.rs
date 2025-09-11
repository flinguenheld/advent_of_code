use std::collections::HashMap;

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";
    // let path = "input_example_2.txt";

    let mut bounds: HashMap<&str, Vec<&str>> = HashMap::new();
    let input = std::fs::read_to_string(path).unwrap();
    for line in input.lines() {
        if let Some((from, to)) = line.split_once('-') {
            bounds
                .entry(from)
                .and_modify(|e| e.push(to))
                .or_insert(vec![to]);

            if from != "start" && to != "end" {
                bounds
                    .entry(to)
                    .and_modify(|e| e.push(from))
                    .or_insert(vec![from]);
            }
        }
    }

    println!("Part one: {}", count_paths(&bounds, vec!["start"], false));
    println!("Part two: {}", count_paths(&bounds, vec!["start"], true));
}

fn count_paths<'a>(
    bounds: &'a HashMap<&'a str, Vec<&'a str>>,
    current: Vec<&'a str>,
    part_two: bool,
) -> u32 {
    let mut count = 0;

    for cave in bounds.get(current.last().unwrap()).unwrap() {
        if *cave == "end" {
            count += 1;
        } else if *cave != "start" {
            if cave.chars().all(|c| c.is_ascii_lowercase())
                && ((!part_two && current.contains(cave))
                    || (part_two
                        && current.contains(cave)
                        && current
                            .iter()
                            .filter(|c| c.chars().all(|ca| ca.is_ascii_lowercase()))
                            .any(|c| current.iter().filter(|ca| *ca == c).count() == 2)))
            {
                continue;
            } else {
                let mut new = current.clone();
                new.push(cave);
                count += count_paths(bounds, new, part_two);
            }
        }
    }
    count
}
