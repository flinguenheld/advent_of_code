fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let input: Vec<Vec<u32>> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|f| f.parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    println!(
        "Part one: {}",
        input.iter().fold(0, |acc, sides| {
            if sides[0] + sides[1] > sides[2]
                && sides[1] + sides[2] > sides[0]
                && sides[2] + sides[0] > sides[1]
            {
                acc + 1
            } else {
                acc
            }
        })
    );

    println!(
        "Part two: {}",
        input.windows(3).step_by(3).fold(0, |acc, win| {
            let mut valid = 0;
            for i in 0..3 {
                if win[0][i] + win[1][i] > win[2][i]
                    && win[1][i] + win[2][i] > win[0][i]
                    && win[2][i] + win[0][i] > win[1][i]
                {
                    valid += 1;
                }
            }
            acc + valid
        })
    );
}
