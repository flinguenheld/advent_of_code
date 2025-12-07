use std::{collections::HashMap, mem};

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let splitters: Vec<(i32, i32)> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.char_indices()
                .filter(|(_, c)| *c != '.')
                .map(move |(col, _)| (row as i32, col as i32))
        })
        .collect();

    let mut split_times = 0;
    let mut current_row = 1;
    let bottom = splitters.iter().max_by_key(|(row, _)| row).unwrap().0 + 1;

    let mut tachyons: HashMap<(i32, i32), u64> = HashMap::new();
    tachyons.insert(*splitters.iter().find(|(row, _)| *row == 0).unwrap(), 1);

    while current_row <= bottom {
        let mut temp: HashMap<(i32, i32), u64> = HashMap::new();
        for ((r, c), value) in tachyons.iter() {
            if splitters.contains(&(*r + 1, *c)) {
                split_times += 1;
                temp.entry((r + 1, c - 1))
                    .and_modify(|e| *e += value)
                    .or_insert(*value);
                temp.entry((r + 1, c + 1))
                    .and_modify(|e| *e += value)
                    .or_insert(*value);
            } else {
                temp.entry((r + 1, *c))
                    .and_modify(|e| *e += value)
                    .or_insert(*value);
            }
        }

        current_row += 1;
        mem::swap(&mut tachyons, &mut temp);
    }

    println!("Part one: {}", split_times);
    println!("Part two: {}", tachyons.values().sum::<u64>());
}

#[allow(dead_code)]
fn print_me(splitters: &[(i32, i32)], tachyons: &HashMap<(i32, i32), u64>) {
    let max_row = splitters.iter().max_by_key(|(row, _)| row).unwrap().0 + 1;
    let max_col = splitters.iter().max_by_key(|(_, col)| col).unwrap().1 + 1;

    for row in 0..=max_row {
        for col in 0..=max_col {
            if let Some(value) = tachyons.get(&(row, col)) {
                print!("{}", value % 10);
            } else if splitters.contains(&(row, col)) {
                print!("^");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!("-------------------------------------------------");
    println!("-------------------------------------------------");
}
