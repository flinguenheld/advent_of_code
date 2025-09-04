use std::{cmp, collections::HashMap};

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let mut vents_p1: HashMap<(i32, i32), i32> = HashMap::new();
    let mut vents_p2: HashMap<(i32, i32), i32> = HashMap::new();

    for line in std::fs::read_to_string(path).unwrap().lines() {
        let (x, y) = line.split_once(" -> ").unwrap();

        let x: Vec<i32> = x.split(',').map(|v| v.parse::<i32>().unwrap()).collect();
        let y: Vec<i32> = y.split(',').map(|v| v.parse::<i32>().unwrap()).collect();

        if x[0] == y[0] {
            for i in cmp::min(x[1], y[1])..=cmp::max(x[1], y[1]) {
                vents_p1
                    .entry((i, x[0]))
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }
        } else if x[1] == y[1] {
            for i in cmp::min(x[0], y[0])..=cmp::max(x[0], y[0]) {
                vents_p1
                    .entry((x[1], i))
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }
        } else {
            let mut row = x[1];
            let mut col = x[0];

            for _ in cmp::min(x[0], y[0])..=cmp::max(x[0], y[0]) {
                vents_p2
                    .entry((row, col))
                    .and_modify(|e| *e += 1)
                    .or_insert(1);

                row += if row < y[1] + 1 { 1 } else { -1 };
                col += if col < y[0] + 1 { 1 } else { -1 };
            }
        }
    }

    // print(&vents_p1);
    println!(
        "Part one: {}",
        vents_p1.values().filter(|v| **v > 1).count()
    );

    // --
    for (k, v) in vents_p1.iter() {
        vents_p2.entry(*k).and_modify(|e| *e += *v).or_insert(*v);
    }

    // print(&vents_p2);
    println!(
        "Part two: {}",
        vents_p2.values().filter(|v| **v > 1).count()
    );
}

#[allow(dead_code)]
fn print(vents: &HashMap<(i32, i32), i32>) {
    for row in 0..=vents.keys().max_by_key(|k| k.0).unwrap().0 {
        for col in 0..=vents.keys().max_by_key(|k| k.1).unwrap().1 {
            if let Some(nb) = vents.get(&(row, col)) {
                print!("{}", nb);
            } else {
                print!(".");
            }
        }
        println!();
    }
}
