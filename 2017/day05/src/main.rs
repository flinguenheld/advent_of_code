fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let input: Vec<i32> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();

    let mut counter = 0;
    let mut jumps = input.clone();
    let mut index = 0_i32;
    while let Some(current) = jumps.get_mut(index as usize) {
        counter += 1;
        index += *current;
        *current += 1;
    }
    println!("Part one: {}", counter);

    // --
    let mut counter = 0;
    let mut jumps = input.clone();
    let mut index = 0_i32;
    while let Some(current) = jumps.get_mut(index as usize) {
        counter += 1;
        index += *current;
        *current += match *current >= 3 {
            true => -1,
            false => 1,
        };
    }
    println!("Part two: {}", counter);
}
