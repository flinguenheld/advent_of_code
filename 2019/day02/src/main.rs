fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let input: Vec<usize> = std::fs::read_to_string(path)
        .unwrap()
        .trim()
        .split(',')
        .map(|v| v.parse::<usize>().unwrap())
        .collect();

    let mut input_p1 = input.clone();
    input_p1[1] = 12;
    input_p1[2] = 2;

    println!("Part one: {}", run(input_p1));

    // --
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut input = input.clone();

            input[1] = noun;
            input[2] = verb;

            if run(input) == 19690720 {
                println!("Part two: {}", 100 * noun + verb);
            }
        }
    }
}

fn run(mut input: Vec<usize>) -> usize {
    let mut index = 0;
    loop {
        if input[index] == 99 {
            break;
        } else {
            let a = input[index + 1];
            let b = input[index + 2];
            let target = input[index + 3];

            input[target] = match input[index] {
                1 => input[a] + input[b],
                _ => input[a] * input[b],
            };
        }

        index += 4;
    }

    input[0]
}
