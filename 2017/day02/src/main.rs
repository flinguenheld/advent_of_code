fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    println!(
        "Part one: {}",
        std::fs::read_to_string(path)
            .unwrap()
            .lines()
            .fold(0, |acc, line| {
                let numbers: Vec<u32> = line
                    .split(|c: char| !c.is_ascii_digit())
                    .map(|num| num.parse::<u32>().unwrap())
                    .collect();

                acc + numbers.iter().max().unwrap() - numbers.iter().min().unwrap()
            })
    );

    println!(
        "Part two: {}",
        std::fs::read_to_string(path)
            .unwrap()
            .lines()
            .fold(0, |acc, line| {
                let mut numbers: Vec<u32> = line
                    .split(|c: char| !c.is_ascii_digit())
                    .map(|num| num.parse::<u32>().unwrap())
                    .collect();
                numbers.sort();

                for (i, a) in numbers.iter().rev().enumerate() {
                    for b in numbers.iter().rev().skip(i + 1) {
                        if a.rem_euclid(*b) == 0 {
                            return acc + a / b;
                        }
                    }
                }

                acc
            })
    )
}
