use std::collections::HashMap;

fn main() {
    let path = "input.txt";

    let aunty_tape = std::fs::read_to_string("aunty_tape.txt").unwrap();
    let aunty_tape: HashMap<&str, &str> = aunty_tape
        .lines()
        .map(|line| {
            let (key, value) = line.split_once(": ").unwrap();
            (key, value)
        })
        .collect();

    'pouet: for line in std::fs::read_to_string(path).unwrap().lines() {
        if let Some((sue_id, fields)) = line.split_once(": ") {
            for field in fields.split(", ") {
                if let Some((key, value)) = field.split_once(": ") {
                    if let Some(aunty_value) = aunty_tape.get(key) {
                        if *aunty_value != value {
                            continue 'pouet;
                        }
                    }
                }
            }

            println!("Part one: {}", sue_id.split_whitespace().last().unwrap());
            break;
        }
    }

    // --
    'pouet: for line in std::fs::read_to_string(path).unwrap().lines() {
        if let Some((sue_id, fields)) = line.split_once(": ") {
            for field in fields.split(", ") {
                if let Some((key, value)) = field.split_once(": ") {
                    if let Some(aunty_value) = aunty_tape.get(key) {
                        match key {
                            "cats" | "trees" => {
                                if *aunty_value >= value {
                                    continue 'pouet;
                                }
                            }
                            "pomeranians" | "goldfish" => {
                                if *aunty_value <= value {
                                    continue 'pouet;
                                }
                            }
                            _ => {
                                if *aunty_value != value {
                                    continue 'pouet;
                                }
                            }
                        }
                    }
                }
            }

            println!("Part two: {}", sue_id.split_whitespace().last().unwrap());
            break;
        }
    }
}
