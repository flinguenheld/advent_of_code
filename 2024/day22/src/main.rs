use std::{collections::HashMap, fs};

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let input = fs::read_to_string(path).unwrap();

    let input: Vec<i64> = input
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect();

    let mut sum: i64 = 0;
    for number in input.iter() {
        sum += generate_secret(*number, 2000, false).last().unwrap();
    }
    println!("Part one: {}", sum);

    // Part two -- FAIL: 2450 IS TOO HIGH :'(
    let mut banana_counter: HashMap<Vec<i64>, i64> = HashMap::new();

    // List all the secret numbers for each start with a copy of each start
    let all_secret_numbers: Vec<Vec<i64>> = input
        .iter()
        .map(|nb| generate_secret(*nb, 2000, true))
        .collect();

    // Replace the number by hash of sequences with their value
    let mut all_sequences_per_list: Vec<HashMap<Vec<i64>, i64>> = Vec::new();

    for list in all_secret_numbers.iter() {
        let mut temp = HashMap::new();
        for window in list.windows(5) {
            let value = window[4];
            let sequence: Vec<i64> = window.windows(2).map(|w| w[1] - w[0]).collect();

            // Mandatory to keep the highest value
            temp.entry(sequence)
                .and_modify(|e| {
                    if value > *e {
                        *e = value
                    }
                })
                .or_insert(value);
        }
        all_sequences_per_list.push(temp);
    }

    // Loop in each list and extract sequences one by one
    for list in all_sequences_per_list.iter() {
        // From this one, loop in all lists again to find if this sequence exists
        for sequence in list.iter() {
            if !banana_counter.contains_key(sequence.0) {
                for list_again in all_sequences_per_list.iter() {
                    if let Some(value) = list_again.get(sequence.0) {
                        banana_counter
                            .entry(sequence.0.clone())
                            .and_modify(|e| *e += *value)
                            .or_insert(*value);
                    }
                }
            }
        }
    }

    // dbg!(&banana_counter);
    println!(
        "Part two: {:?}",
        banana_counter
            .iter()
            .max_by(|a, b| a.1.cmp(b.1))
            .map(|(_, v)| v)
    );
}

fn generate_secret(mut secret_number: i64, times: u32, last_digit_only: bool) -> Vec<i64> {
    let mut output = if last_digit_only {
        vec![secret_number % 10]
    } else {
        vec![secret_number]
    };

    for _ in 0..times {
        // Modulo = AND divisor-1
        secret_number = (secret_number ^ (secret_number << 6)) & 0xFFFFFF;
        secret_number = (secret_number ^ (secret_number >> 5)) & 0xFFFFFF;
        secret_number = (secret_number ^ (secret_number << 11)) & 0xFFFFFF;
        if last_digit_only {
            output.push(secret_number % 10);
        } else {
            output.push(secret_number);
        }
    }
    output
}
