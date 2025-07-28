use std::{collections::HashMap, mem::swap};

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let input = std::fs::read_to_string(path).unwrap();

    let mut acres: HashMap<(i32, i32), char> = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.char_indices()
                .map(move |(col, c)| ((row as i32, col as i32), c))
        })
        .collect();

    let mut previous_states = vec![acres.clone()];
    // print(&acres);

    let mut cache = HashMap::new();

    for i in 0..1_000_000_000 {
        for point in acres.keys() {
            cache.insert(*point, new_state(point, &acres));
        }

        swap(&mut cache, &mut acres);

        // --
        if i == (10 - 1) {
            println!("Part one: {}", lumber_collection(&acres));
        }

        // The pattern loops, so find when and the frequency
        if let Some(start) = previous_states.iter().position(|a| *a == acres) {
            let gap = (i - start) + 1;

            let the_one = previous_states[(1_000_000_000 - start) % gap + start].clone();
            println!("Part two: {}", lumber_collection(&the_one));

            break;
        }

        previous_states.push(acres.clone());
    }
}

fn lumber_collection(acres: &HashMap<(i32, i32), char>) -> usize {
    acres.iter().filter(|(_, v)| **v == '|').count()
        * acres.iter().filter(|(_, v)| **v == '#').count()
}

fn new_state(current: &(i32, i32), acres: &HashMap<(i32, i32), char>) -> char {
    let mut neighbours = Vec::new();
    for row in -1..=1 {
        for col in -1..=1 {
            if !(row == 0 && col == 0) {
                if let Some(n) = acres.get(&(current.0 + row, current.1 + col)) {
                    neighbours.push(*n);
                }
            }
        }
    }

    let c = acres.get(current).unwrap();

    if *c == '#' {
        if neighbours.iter().filter(|n| **n == '#').count() >= 1
            && neighbours.iter().filter(|n| **n == '|').count() >= 1
        {
            return '#';
        } else {
            return '.';
        }
    } else if *c == '|' {
        if neighbours.iter().filter(|n| **n == '#').count() >= 3 {
            return '#';
        }
    } else if neighbours.iter().filter(|n| **n == '|').count() >= 3 {
        return '|';
    }

    *c
}

#[allow(dead_code)]
fn print(acres: &HashMap<(i32, i32), char>) {
    let row_max = acres.keys().max_by_key(|(row, _)| row).unwrap().0;
    let col_max = acres.keys().max_by_key(|(_, col)| col).unwrap().1;

    for row in 0..row_max + 1 {
        for col in 0..col_max + 1 {
            if let Some(acre) = acres.get(&(row, col)) {
                print!("{}", acre);
            }
        }
        println!();
    }
}
