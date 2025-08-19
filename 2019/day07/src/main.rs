use itertools::Itertools;
use std::mem;

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let instructions: Vec<i64> = std::fs::read_to_string(path)
        .unwrap()
        .trim()
        .split(',')
        .map(|v| v.parse::<i64>().unwrap())
        .collect();

    let mut all: Vec<i64> = Vec::new();
    for combination in [0, 1, 2, 3, 4].iter().permutations(5) {
        let mut last_code = 0;
        for i in combination.iter() {
            last_code = get_last_output(instructions.clone(), (**i, last_code));
        }
        all.push(last_code);
    }

    println!("Part one: {}", all.iter().max().unwrap());
}

fn get_last_output(mut instructions: Vec<i64>, mut inputs: (i64, i64)) -> i64 {
    let mut index = 0;
    let mut output = 0;
    while index != usize::MAX {
        index = run(&mut instructions, index, &mut inputs, &mut output);
    }

    output
}

fn run(
    instructions: &mut [i64],
    mut index: usize,
    inputs: &mut (i64, i64),
    output: &mut i64,
) -> usize {
    let (opcode, mode_1, mode_2, _) = get_orders(instructions[index]);

    if opcode == 99 {
        index = usize::MAX;
    } else if opcode == 3 {
        instructions[instructions[index + 1] as usize] = inputs.0;
        mem::swap(&mut inputs.0, &mut inputs.1);
        index += 2;
    } else if opcode == 4 {
        if instructions[instructions[index + 1] as usize] != 0 {
            *output = instructions[instructions[index + 1] as usize];
        }
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
