use std::collections::HashMap;

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let input = std::fs::read_to_string(path).unwrap();

    let mut total = 0;
    for line in input.lines() {
        let (left, right) = line.split_once('[').unwrap();

        let checksum = right[..5].to_string();
        let id = left
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<u32>()
            .unwrap();

        // Count letters
        let mut letters: HashMap<char, u32> = HashMap::new();
        for c in left.chars().filter(|c| c.is_ascii_alphabetic()) {
            letters.entry(c).and_modify(|e| *e += 1).or_insert(1);
        }

        if checksum.chars().all(|c| {
            if let Some(value) = letters.get(&c) {
                if value == letters.iter().max_by_key(|e| e.1).unwrap().1 {
                    letters.remove_entry(&c);
                    return true;
                }
            }
            false
        }) {
            total += id;

            // Part two --
            let name = left
                .chars()
                .filter(|c| !c.is_ascii_digit())
                .map(|c| match c {
                    '-' => ' ',
                    _ => decrypt(c, id),
                })
                .collect::<String>();

            if name.contains("north") {
                println!("Part two: {} -> ({})", id, name);
            }
        }
    }

    println!("Part one: {}", total);
}

fn decrypt(c: char, id: u32) -> char {
    let mut ascii = (c as u8 - 97) as u32;
    ascii = (ascii + id).rem_euclid(26);

    (ascii + 97) as u8 as char
}
