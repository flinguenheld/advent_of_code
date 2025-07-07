fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let input: Vec<u8> = std::fs::read_to_string(path)
        .unwrap()
        .trim()
        .chars()
        .map(|c| c as u8)
        .collect();

    println!("Part one: {}", react(input.clone()));

    // --
    let mut shortest = usize::MAX;
    for poly in 65..=90 {
        if input.contains(&poly) {
            let input: Vec<u8> = input
                .iter()
                .filter(|c| **c != poly && **c != poly + 32)
                .cloned()
                .collect();

            shortest = shortest.min(react(input));
        }
    }
    println!("Part two: {}", shortest);
}

fn react(mut input: Vec<u8>) -> usize {
    let mut index = 0;
    while index < input.len() - 1 {
        if input[index].abs_diff(input[index + 1]) == 32 {
            input.remove(index + 1);
            input.remove(index);
            index = index.saturating_sub(1);
            continue;
        }
        index += 1;
    }

    input.len()
}
