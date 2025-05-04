use std::collections::HashMap;

fn main() {
    // let input = "1,3,2";
    // let input = "0,3,6";
    let input = "8,0,17,4,1,12";

    let input = input
        .split(',')
        .rev()
        .map(|v| v.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    println!("Part one: {}", play(input.clone(), 2020));
    println!("Part two: {}", play(input, 30_000_000));
}

fn play(mut input: Vec<u32>, turns: u32) -> u32 {
    let mut memory: HashMap<u32, u32> = HashMap::new();
    let mut last_spoken = 0;
    let mut current = 0;

    for i in 1..=turns {
        last_spoken = current;

        if let Some(input_value) = input.pop() {
            memory.insert(input_value, i);
            current = 0;
        } else if let Some(current_key_value) = memory.get_mut(&current) {
            current = i - *current_key_value;
            *current_key_value = i;
        } else {
            memory.insert(current, i);
            current = 0;
        }
    }

    last_spoken
}
