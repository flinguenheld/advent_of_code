use std::collections::{HashMap, HashSet};

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let input = std::fs::read_to_string(path).unwrap();

    let mut fabric: HashMap<(u32, u32), HashSet<u32>> = HashMap::new();

    for line in input.lines() {
        let fields: Vec<&str> = line.split_whitespace().collect();

        let id = to_num(fields[0]);

        let (space_left, space_top) = fields[2].split_once(',').unwrap();
        let space_left = to_num(space_left);
        let space_top = to_num(space_top);

        let (width, height) = fields[3].split_once('x').unwrap();
        let width = to_num(width);
        let height = to_num(height);

        for row in space_top..(space_top + height) {
            for col in space_left..(space_left + width) {
                fabric
                    .entry((row, col))
                    .and_modify(|e| {
                        e.insert(id);
                    })
                    .or_insert(HashSet::from([id]));
            }
        }
    }

    println!(
        "Part one: {}",
        fabric.values().filter(|v| v.len() > 1).count()
    );

    // --
    // Get all square inches which are alone
    let mut alones: HashSet<u32> = fabric
        .values()
        .filter(|v| v.len() == 1)
        .map(|v| v.iter().next().unwrap())
        .cloned()
        .collect();

    // Remove all of them which are also in another groups
    alones.retain(|v| {
        !fabric
            .values()
            .filter(|group| group.len() > 1)
            .any(|group| group.contains(v))
    });

    println!("Part two: {}", alones.iter().next().unwrap());
}

fn to_num(txt: &str) -> u32 {
    txt.chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<u32>()
        .unwrap()
}
