use std::collections::HashMap;

fn generate_input() -> HashMap<u64, u64> {
    #[cfg(feature = "example")]
    let input = "0 1 10 99 999";

    #[cfg(not(feature = "example"))]
    let input = "1 24596 0 740994 60 803 8918 9405859";

    input
        .split_whitespace()
        .map(|v| (v.parse::<u64>().unwrap(), 1))
        .collect()
}

fn main() {
    let mut input = generate_input();
    for _ in 0..25 {
        input = blink(input);
    }
    println!("Part one: {}", input.values().sum::<u64>());

    let mut input = generate_input();
    for _ in 0..75 {
        input = blink(input);
    }
    println!("Part two: {}", input.values().sum::<u64>());
}

fn blink(input: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut brand_new: HashMap<u64, u64> = HashMap::new();

    for (value, times) in input.iter() {
        let to_string = value.to_string();

        let keys = if *value == 0 {
            vec![1]
        } else if *value > 9 && to_string.chars().count() % 2 == 0 {
            vec![
                to_string[..(to_string.len() / 2)].parse::<u64>().unwrap(),
                to_string[(to_string.len() / 2)..].parse::<u64>().unwrap(),
            ]
        } else {
            vec![value * 2024]
        };

        for k in keys {
            brand_new
                .entry(k)
                .and_modify(|e| *e += *times)
                .or_insert(*times);
        }
    }
    brand_new
}
