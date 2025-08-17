fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let mut instructions: Vec<i64> = std::fs::read_to_string(path)
        .unwrap()
        .trim()
        .split(',')
        .map(|v| v.parse::<i64>().unwrap())
        .collect();

    let mut index = 0;
    while index != usize::MAX {
        index = run(&mut instructions, index, 5);
    }
}

fn run(instructions: &mut [i64], mut index: usize, input: i64) -> usize {
    let (opcode, mode_1, mode_2, _) = get_orders(instructions[index]);

    if opcode == 99 {
        index = usize::MAX;
    } else if opcode == 3 {
        instructions[instructions[index + 1] as usize] = input;
        index += 2;
    } else if opcode == 4 {
        println!(
            "Diagnostic code: {}",
            instructions[instructions[index + 1] as usize]
        );
        index += 2;
    } else {
        // Get parameters according to their modes --
        let a = match mode_1 == 0 {
            true => instructions[instructions[index + 1] as usize],
            false => instructions[index + 1],
        };
        let b = match mode_2 == 0 {
            true => instructions[instructions[index + 2] as usize],
            false => instructions[index + 2],
        };
        let target = instructions[index + 3] as usize;

        // Go --
        if opcode == 5 {
            if a != 0 {
                return b as usize;
            } else {
                index += 3;
            }
        } else if opcode == 6 {
            if a == 0 {
                return b as usize;
            } else {
                index += 3;
            }
        } else if opcode == 7 {
            if a < b {
                instructions[target] = 1;
            } else {
                instructions[target] = 0;
            }
            index += 4;
        } else if opcode == 8 {
            if a == b {
                instructions[target] = 1;
            } else {
                instructions[target] = 0;
            }
            index += 4;
        } else {
            instructions[target] = match opcode {
                1 => a + b,
                _ => a * b,
            };
            index += 4;
        }
    }

    index
}

fn get_orders(mut val: i64) -> (i64, i64, i64, i64) {
    let de = val % 100;
    val /= 100;

    let c = val % 10;
    val /= 10;

    let b = val % 10;
    val /= 10;

    (de, c, b, val % 10)
}
