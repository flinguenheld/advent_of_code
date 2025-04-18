use std::fs;

fn main() {
    // let path = "input_example.txt";
    let path = "input.txt";

    let input: Vec<i32> = fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| {
            if let Some((_, value)) = l.split_once(' ') {
                value.parse::<i32>().unwrap()
            } else {
                0
            }
        })
        .collect();

    let mut values: [(i32, Option<i32>); 6] = [
        (20, None),
        (60, None),
        (100, None),
        (140, None),
        (180, None),
        (220, None),
    ];
    let mut cycle = 0;
    let mut x = 1;

    for cmd in input.iter() {
        if *cmd == 0 {
            cycle = add_cycle(cycle, x, &mut values);
        } else {
            cycle = add_cycle(cycle, x, &mut values);
            cycle = add_cycle(cycle, x, &mut values);
            x += cmd;
        }
    }

    println!(
        "Part one: {}",
        values.iter().fold(0, |acc, (c, v)| acc + (c * v.unwrap()))
    );

    // --
    let mut iter = input.into_iter().peekable();
    let mut rows = [['.'; 40]; 6];

    let mut current_position = 1;
    let mut wait = false;

    for row in rows.iter_mut() {
        for cycle in 0..40 {
            if let Some(&cmd) = iter.peek() {
                if cycle == current_position - 1
                    || cycle == current_position
                    || cycle == current_position + 1
                {
                    row[cycle as usize] = '#';
                }

                if cmd == 0 {
                    iter.next();
                } else {
                    if wait {
                        current_position += cmd;
                        iter.next();
                    }
                    wait = !wait;
                }
            }
        }
    }

    println!("Part two:");
    for row in rows.iter() {
        for c in row.iter() {
            print!("{} ", c);
        }
        println!();
    }
}

fn add_cycle(current: i32, x: i32, values: &mut [(i32, Option<i32>); 6]) -> i32 {
    if let Some((_, value)) = values.iter_mut().find(|(cycle, _)| *cycle == current + 1) {
        *value = Some(x);
    }

    current + 1
}
