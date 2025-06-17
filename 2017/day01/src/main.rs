fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let mut input: Vec<u32> = std::fs::read_to_string(path)
        .unwrap()
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c as u32 - 48)
        .collect();
    input.push(*input.first().unwrap());

    println!(
        "Part one: {}",
        input.windows(2).fold(0, |acc, win| match win[0] == win[1] {
            true => acc + win[0],
            false => acc,
        })
    );

    input.pop();

    println!(
        "Part two: {}",
        input.iter().enumerate().fold(0, |acc, (i, val)| {
            match *val == input[(i + input.len() / 2).rem_euclid(input.len())] {
                true => acc + val,
                false => acc,
            }
        })
    )
}
