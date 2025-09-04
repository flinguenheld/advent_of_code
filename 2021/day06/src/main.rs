use std::collections::VecDeque;

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let mut fish: VecDeque<u64> = VecDeque::from([0; 9]);

    for value in std::fs::read_to_string(path).unwrap().trim().split(',') {
        fish[value.parse().unwrap()] += 1;
    }

    for day in 1..=256 {
        let zero = fish.pop_front().unwrap();
        fish.push_back(zero);
        fish[6] += zero;

        if day == 80 {
            println!("Part one: {}", fish.iter().sum::<u64>());
        }
    }

    println!("Part two: {}", fish.iter().sum::<u64>());
}
