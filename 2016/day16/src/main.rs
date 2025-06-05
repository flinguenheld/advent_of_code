fn main() {
    // let (input, length) = ("10000", 20);
    let (input, length) = ("10010000000110000", 272);
    println!("Part one: {}", checksum(input, length));

    let (input, length) = ("10010000000110000", 35651584);
    println!("Part two: {}", checksum(input, length));
}

fn checksum(input: &str, length: usize) -> String {
    let mut input = input.chars().collect::<Vec<char>>();

    // Dragonise until length has been reached
    while input.len() < length {
        let mut new: Vec<char> = input
            .iter()
            .rev()
            .map(|c| match c {
                '1' => '0',
                _ => '1',
            })
            .collect();
        input.push('0');
        input.append(&mut new);
    }

    let mut dragon: Vec<char> = input[..length].to_vec();

    while dragon.len() == length || dragon.len() % 2 == 0 {
        dragon = dragon
            .windows(2)
            .step_by(2)
            .map(|w| match w {
                ['1', '1'] | ['0', '0'] => '1',
                _ => '0',
            })
            .collect();
    }

    dragon.iter().collect()
}
