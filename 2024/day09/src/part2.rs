use std::fs;

#[derive(Clone, Copy)]
struct Slot {
    value: u64,
    times: u64,
    done: bool,
}

fn main() {
    // let path = "input.txt";
    let path = "input_example.txt";

    let input: Vec<u8> = fs::read(path).unwrap();
    let input: Vec<u64> = input
        .iter()
        .filter(|c| **c >= 48)
        .map(|c| *c as u64 - 48)
        .collect();

    let mut current_value = -1;
    let mut input: Vec<Slot> = input
        .iter()
        .enumerate()
        .map(|(i, val)| {
            if i % 2 == 0 {
                current_value += 1;
                Slot {
                    value: current_value as u64,
                    times: *val,
                    done: false,
                }
            } else {
                Slot {
                    value: u64::MAX,
                    times: *val,
                    done: false,
                }
            }
        })
        .collect();
    // display(&input);

    'aaa: loop {
        for right in (0..input.len()).rev() {
            if input[right].value < u64::MAX && !input[right].done {
                for left in 0..input.len() {
                    if left == right {
                        break;
                    }

                    if input[left].value == u64::MAX && input[left].times >= input[right].times {
                        let difference = input[left].times - input[right].times;

                        input[left].value = input[right].value;
                        input[left].times = input[right].times;
                        input[right].value = u64::MAX;

                        if difference > 0 {
                            input.insert(
                                left + 1,
                                Slot {
                                    value: u64::MAX,
                                    times: difference,
                                    done: false,
                                },
                            );
                        }
                        continue 'aaa;
                    }
                }
                input[right].done = true;
                continue 'aaa;
            }
        }
        break 'aaa;
    }

    let mut checksum = 0;
    let mut current_mul = 0;
    for slot in input.iter() {
        for _ in 0..slot.times {
            if slot.value < u64::MAX {
                checksum += slot.value * current_mul;
            }
            current_mul += 1;
        }
    }

    display(&input);
    println!("Part two: {}", checksum);
}

fn display(input: &[Slot]) {
    for slot in input.iter() {
        for _ in 0..slot.times {
            if slot.value == u64::MAX {
                print!(".");
            } else {
                print!("{}", slot.value);
            }
        }
    }
    println!();
}
