use std::collections::HashMap;

fn main() {
    // let path = "input_example.txt";
    let path = "input.txt";

    let input = std::fs::read_to_string(path).unwrap();
    let mut input: Vec<&str> = input.lines().collect();

    let a = proceed(input.clone());
    println!("Part one: {}", a);

    // --
    let replace_b = format!("{} -> b", a);
    input.retain(|l| !l.ends_with(" -> b"));
    input.insert(0, &replace_b);

    let a = proceed(input);
    println!("Part two: {}", a);
}

fn proceed(mut input: Vec<&str>) -> u16 {
    let mut values: HashMap<&str, u16> = HashMap::new();
    while !input.is_empty() {
        input.retain(|line| {
            if let Some((operation, to)) = line.split_once(" -> ") {
                let parts: Vec<&str> = operation.split_whitespace().collect();

                if parts.len() == 1 {
                    if let Some(v) = get_value(parts.first().unwrap(), &values) {
                        values.insert(to, v);
                        return false;
                    }
                } else if parts.len() == 2 {
                    if let Some(v) = values.get(parts.last().unwrap()) {
                        values.insert(to, !v);
                        return false;
                    }
                } else if let Some(left) = get_value(parts.first().unwrap(), &values) {
                    if let Some(right) = get_value(parts.last().unwrap(), &values) {
                        values.insert(
                            to,
                            match *parts.get(1).unwrap() {
                                "AND" => left & right,
                                "OR" => left | right,
                                "LSHIFT" => left << right,
                                _ => left >> right,
                            },
                        );
                        return false;
                    }
                }
            }
            true
        });
    }

    *values.get("a").unwrap_or(&0)
}

// A value can be a number or a key and the entry maybe hasn't been created yet.
fn get_value(key: &str, values: &HashMap<&str, u16>) -> Option<u16> {
    if let Ok(v) = key.parse::<u16>() {
        Some(v)
    } else {
        values.get(key).copied()
    }
}
