use std::collections::{HashMap, VecDeque};

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let input = std::fs::read_to_string(path).unwrap();
    let input: Vec<(&str, &str)> = input.lines().map(|l| l.split_once(')').unwrap()).collect();

    let mut orbits: HashMap<&str, u32> = HashMap::new();
    let mut queue: VecDeque<(&str, u32)> = VecDeque::new();

    // Get the start
    for i in 0..input.len() {
        if input
            .iter()
            .filter(|line| line.0 == input[i].0 || line.1 == input[i].0)
            .count()
            == 1
        {
            orbits.insert(input[i].0, 0);
            queue.push_back((input[i].0, 0));
        }
    }

    // List all orbits with their value
    while let Some(current) = queue.pop_front() {
        orbits.insert(current.0, current.1);

        for pair in input.iter().filter(|p| p.0 == current.0) {
            queue.push_back((pair.1, current.1 + 1));
        }
    }

    println!("Part one: {}", orbits.values().sum::<u32>());

    // --
    // Find the common point
    let you_to_start = get_path_to_start("YOU", &input);
    let san_to_start = get_path_to_start("SAN", &input);

    for (dist_you_common, common) in you_to_start.iter().enumerate() {
        if san_to_start.contains(common) {
            let dist_san_common = san_to_start.iter().position(|o| o == common).unwrap();

            println!("Part two: {}", dist_you_common + dist_san_common - 2);
            break;
        }
    }
}

fn get_path_to_start<'a>(who: &'a str, input: &'a [(&'a str, &'a str)]) -> Vec<&'a str> {
    let mut path: Vec<&str> = vec![who];
    while let Some(or) = input.iter().find(|pair| pair.1 == *path.last().unwrap()) {
        path.push(or.0);
    }
    path
}
