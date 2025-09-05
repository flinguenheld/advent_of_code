use std::collections::HashMap;

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let input: Vec<u32> = std::fs::read_to_string(path)
        .unwrap()
        .trim()
        .split(',')
        .map(|v| v.parse().unwrap())
        .collect();

    let fuel_sums: Vec<(u32, u32)> = input
        .iter()
        .map(|a| {
            (
                *a,
                input
                    .iter()
                    .filter(|b| *b != a)
                    .fold(0, |acc, b| acc + a.abs_diff(*b)),
            )
        })
        .collect();

    println!(
        "Part one: {}",
        fuel_sums.iter().min_by_key(|(_, fuel)| fuel).unwrap().1
    );

    // --
    let mut sum = 0;
    let mem: HashMap<u32, u32> = (1..=*input.iter().max().unwrap())
        .map(|v| {
            sum += v;
            (v, sum)
        })
        .collect();

    let fuel_sums: Vec<(u32, u32)> = (0..*input.iter().max().unwrap())
        .map(|a| {
            (
                a,
                input
                    .iter()
                    .filter(|b| **b != a)
                    .fold(0, |acc, b| acc + mem.get(&a.abs_diff(*b)).unwrap()),
            )
        })
        .collect();

    println!(
        "Part two: {}",
        fuel_sums.iter().min_by_key(|(_, fuel)| fuel).unwrap().1
    );
}
