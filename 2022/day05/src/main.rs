use std::{collections::VecDeque, fs};

fn main() {
    // let path = "input_example.txt";
    let path = "input.txt";

    let input = fs::read_to_string(path).unwrap();
    let nb_stacks = input
        .lines()
        .find(|l| l.starts_with(" 1"))
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    // Build current stacks --
    let mut stacks: Vec<VecDeque<char>> = vec![VecDeque::new(); nb_stacks];
    let mut stacks2: Vec<VecDeque<char>> = vec![VecDeque::new(); nb_stacks];
    for line in input.lines().filter(|l| l.contains('[')) {
        for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
            if !c.is_whitespace() {
                stacks[i].push_front(c);
                stacks2[i].push_front(c);
            }
        }
    }
    display(&stacks);

    // Apply all instructions --
    for line in input.lines().filter(|l| l.starts_with("move")) {
        let instructions: Vec<usize> = line
            .split_whitespace()
            .filter_map(|f| match f.parse::<usize>() {
                Ok(n) => Some(n),
                _ => None,
            })
            .collect();

        // Part one --
        (0..instructions[0]).for_each(|_| {
            if let Some(value) = stacks[instructions[1] - 1].pop_back() {
                stacks[instructions[2] - 1].push_back(value);
            }
        });

        // Part two --
        let mut new_pile: VecDeque<char> = (0..instructions[0])
            .filter_map(|_| stacks2[instructions[1] - 1].pop_back())
            .collect();

        while let Some(value) = new_pile.pop_back() {
            stacks2[instructions[2] - 1].push_back(value);
        }
    }

    display(&stacks);
    println!(
        "Part one: {}",
        stacks.iter().filter_map(|s| s.back()).collect::<String>()
    );

    display(&stacks2);
    println!(
        "Part two: {}",
        stacks2.iter().filter_map(|s| s.back()).collect::<String>()
    );
}

fn display(stacks: &[VecDeque<char>]) {
    println!("------------------------------------------------------");
    for row in (0..stacks
        .iter()
        .max_by(|&l, &r| l.len().cmp(&r.len()))
        .unwrap()
        .len())
        .rev()
    {
        for c in stacks.iter() {
            if let Some(v) = c.get(row) {
                print!("[{}]", v);
            } else {
                print!("   ");
            }
            print!(" ");
        }
        println!();
    }
}
