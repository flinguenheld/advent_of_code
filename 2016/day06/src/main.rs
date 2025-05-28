use std::collections::HashMap;

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let input = std::fs::read_to_string(path).unwrap();

    let mut records: Vec<HashMap<char, u32>> =
        vec![HashMap::new(); input.lines().last().unwrap().chars().count()];

    for line in std::fs::read_to_string(path).unwrap().lines() {
        for (i, c) in line.char_indices() {
            records[i].entry(c).and_modify(|e| *e += 1).or_insert(1);
        }
    }

    println!(
        "Part one: {}",
        records
            .iter()
            .map(|r| r.iter().max_by_key(|(_, &v)| v).unwrap().0)
            .collect::<String>()
    );

    println!(
        "Part two: {}",
        records
            .iter()
            .map(|r| r.iter().min_by_key(|(_, &v)| v).unwrap().0)
            .collect::<String>()
    );
}
