fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    println!(
        "Part one: {}",
        std::fs::read_to_string(path)
            .unwrap()
            .lines()
            .fold(0, |acc, line| {
                let v = line.parse::<u32>().unwrap();

                acc + v / 3 - 2
            })
    );

    println!(
        "Part two: {}",
        std::fs::read_to_string(path)
            .unwrap()
            .lines()
            .fold(0, |acc, line| {
                let mut add = 0;
                let mut v = line.parse::<i32>().unwrap();
                loop {
                    v = v / 3 - 2;
                    match v > 0 {
                        true => add += v,
                        false => break,
                    }
                }

                acc + add
            })
    );
}
