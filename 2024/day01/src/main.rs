use std::fs;

fn main() {
    // let file = "input_01_example.txt";
    let file = "input_01.txt";

    let (mut left, mut right) = fs::read_to_string(file)
        .unwrap()
        .lines()
        .map(|line| {
            let mut vals = line.split_whitespace();
            (
                vals.next().unwrap().parse::<u32>().unwrap(),
                vals.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .collect::<(Vec<u32>, Vec<u32>)>();

    // Part one --
    left.sort();
    right.sort();

    let tot = left.iter().zip(right.iter()).fold(
        0,
        |acc, (l, r)| if l <= r { acc + r - l } else { acc + l - r },
    );

    println!("Part one : {}", tot);

    // Part two --
    let tot = left.iter().fold(0, |acc, l| {
        acc + l * (right.iter().filter(|v| *v == l).count() as u32)
    });

    println!("Part two : {}", tot);
}
