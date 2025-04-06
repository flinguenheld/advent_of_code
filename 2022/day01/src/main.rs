use std::fs;

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let mut total = [0; 3];
    let mut temp = 0;
    for line in fs::read_to_string(path).unwrap().lines() {
        if let Ok(nb) = line.parse::<u32>() {
            temp += nb;
        } else {
            if temp > total[0] {
                total[0] = temp;
                total.sort();
            }
            temp = 0;
        }
    }

    println!("Part one: {}", total[2]);
    println!("Part two: {}", total.iter().sum::<u32>());
}
