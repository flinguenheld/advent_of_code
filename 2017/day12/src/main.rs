use std::collections::HashMap;

fn main() {
    // let path = "input.txt";
    let path = "input_example.txt";

    let input = std::fs::read_to_string(path).unwrap();
    let input: HashMap<&str, Vec<&str>> = input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(" <-> ").unwrap();
            let right: Vec<&str> = right
                .split(|c: char| !c.is_ascii_digit())
                .filter(|txt| !txt.is_empty())
                .collect();

            (left, right)
        })
        .collect();

    let mut groups: Vec<Vec<&str>> = Vec::new();

    // Find a key alone
    while let Some((alone, _)) = input
        .iter()
        .find(|(k, _)| !groups.iter().flatten().any(|val| *k == val))
    {
        // Then fill the group
        let mut new_group = vec![*alone];

        // Loop until there's new push
        let mut again = true;
        while again {
            again = false;
            for (key, values) in input.iter() {
                if !new_group.contains(key) && values.iter().any(|v| new_group.contains(v)) {
                    new_group.push(key);
                    again = true;
                }
            }
        }
        groups.push(new_group);
    }

    println!(
        "Part one: {}",
        groups.iter().find(|g| g.contains(&"0")).unwrap().len()
    );
    println!("Part two: {}", groups.len());
}
