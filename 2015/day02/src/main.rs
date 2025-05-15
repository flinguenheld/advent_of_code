use std::fs;

fn main() {
    // let path = "input_example.txt";
    let path = "input.txt";

    let input = fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| {
            let mut val = l
                .split('x')
                .map(|v| v.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            val.sort();
            val
        })
        .collect::<Vec<Vec<u32>>>();

    println!(
        "Part one: {}",
        input.iter().fold(0, |acc, val| {
            acc + 2 * val[0] * val[1] + 2 * val[1] * val[2] + 2 * val[0] * val[2] + val[0] * val[1]
        })
    );

    println!(
        "Part two: {}",
        input.iter().fold(0, |acc, val| {
            acc + 2 * val[0] + 2 * val[1] + val.iter().product::<u32>()
        })
    );
}
