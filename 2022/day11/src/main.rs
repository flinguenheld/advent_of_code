use std::{fs, mem::swap};

struct Monkey {
    items: Vec<u64>,
    operation: Box<dyn Fn(u64) -> u64>,
    test: u64,
    to_ok: usize,
    to_nok: usize,
    count: u64,
}

fn main() {
    let path = "input_example.txt";
    // let path = "input.txt";

    // ----------------------------------------------------------------------------------
    // ---------------------------------------------------------------------  Part one --
    let mut monkeys = parse_input(path);

    for _ in 0..20 {
        for index in 0..monkeys.len() {
            let mut items = Vec::new();
            swap(&mut monkeys[index].items, &mut items);

            let to_ok = monkeys[index].to_ok;
            let to_nok = monkeys[index].to_nok;

            while let Some(mut value) = items.pop() {
                value = (monkeys[index].operation)(value);
                value /= 3;

                if value % monkeys[index].test == 0 {
                    monkeys[to_ok].items.push(value);
                } else {
                    monkeys[to_nok].items.push(value);
                }
                monkeys[index].count += 1;
            }
        }
    }
    monkeys.sort_by_key(|m| m.count);

    println!(
        "Part one: {}",
        monkeys[monkeys.len() - 1].count * monkeys[monkeys.len() - 2].count
    );

    // ----------------------------------------------------------------------------------
    // ---------------------------------------------------------------------  Part two --
    let mut monkeys = parse_input(path);
    let common_multiple: u64 = monkeys.iter().map(|m| m.test).product();

    for _ in 0..10_000 {
        for index in 0..monkeys.len() {
            let mut items = Vec::new();
            swap(&mut monkeys[index].items, &mut items);

            let to_ok = monkeys[index].to_ok;
            let to_nok = monkeys[index].to_nok;

            while let Some(mut value) = items.pop() {
                value = (monkeys[index].operation)(value);

                if value % monkeys[index].test == 0 {
                    monkeys[to_ok].items.push(value % common_multiple);
                } else {
                    monkeys[to_nok].items.push(value % common_multiple);
                }
                monkeys[index].count += 1;
            }
        }
    }

    monkeys.sort_by_key(|m| m.count);
    println!(
        "Part two: {}",
        monkeys[monkeys.len() - 1].count * monkeys[monkeys.len() - 2].count
    );
}

fn parse_input(path: &str) -> Vec<Monkey> {
    let mut monkeys = Vec::new();
    let input = fs::read_to_string(path).unwrap();

    for win in input.lines().collect::<Vec<&str>>().windows(6).step_by(7) {
        // println!("{:?}", line);

        let items: Vec<u64> = win[1][18..]
            .split(", ")
            .map(|v| v.parse::<u64>().unwrap())
            .collect();

        let test = win[3][21..].parse::<u64>().unwrap();

        let (op, value) = win[2][23..].split_once(' ').unwrap();
        let operation: Box<dyn Fn(u64) -> u64> = if op == "+" {
            if let Ok(value) = value.parse::<u64>() {
                Box::new(move |v: u64| v + value)
            } else {
                Box::new(|v: u64| v + v)
            }
        } else if let Ok(value) = value.parse::<u64>() {
            Box::new(move |v: u64| v * value)
        } else {
            Box::new(|v: u64| v * v)
        };

        let to_ok = win[4][29..].parse::<usize>().unwrap();
        let to_nok = win[5][30..].parse::<usize>().unwrap();

        monkeys.push(Monkey {
            items,
            operation,
            test,
            to_ok,
            to_nok,
            count: 0,
        })
    }
    monkeys
}
