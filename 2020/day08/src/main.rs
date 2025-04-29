use std::fs;

#[derive(Debug, Clone, Copy)]
enum Action {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let input: Vec<(Action, bool)> = fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| {
            let (action, value) = l.split_once(' ').unwrap();
            let value = value.parse::<i32>().unwrap();

            (
                match action {
                    "acc" => Action::Acc(value),
                    "jmp" => Action::Jmp(value),
                    _ => Action::Nop(value),
                },
                false,
            )
        })
        .collect();

    println!("Part one: {}", loop_until(&mut input.clone(), 1).unwrap());

    // --
    for (i, action) in input.iter().enumerate() {
        let mut temp = input.clone();
        match action.0 {
            Action::Nop(v) => {
                temp[i] = (Action::Jmp(v), false);
            }
            Action::Jmp(_) => {
                temp[i] = (Action::Nop(0), false);
            }
            Action::Acc(_) => {
                continue;
            }
        }

        if let Some(acc) = loop_until(&mut temp, 2) {
            println!("Part two: {}", acc);
        }
    }
}

fn loop_until(input: &mut [(Action, bool)], part: u8) -> Option<i32> {
    let mut index: i32 = 0;
    let mut acc = 0;
    let len = input.len() as i32;

    loop {
        if let Some((action, state)) = input.get_mut(index as usize) {
            if *state {
                return match part == 1 {
                    true => Some(acc),
                    false => None,
                };
            }

            *state = true;
            match action {
                Action::Acc(v) => {
                    acc += *v;
                    index += 1;
                }
                Action::Jmp(v) => {
                    index += *v;
                }
                Action::Nop(_) => {
                    index += 1;
                }
            }

            if index < 0 || index == len {
                return Some(acc);
            }
        }
    }
}
