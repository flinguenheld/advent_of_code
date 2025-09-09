use std::collections::{HashMap, VecDeque};

const NEIGHBOURS: [(i8, i8); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
];

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";
    // let path = "input_example_2.txt";

    let mut octopus: HashMap<(i8, i8), u8> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.char_indices()
                .map(move |(col, val)| ((row as i8, col as i8), val as u8 - 48))
        })
        .collect();

    // print(&octopus);

    let mut flash_counter = 0;
    for step in 1..1000 {
        octopus.iter_mut().for_each(|(_, v)| *v += 1);

        let mut cache: VecDeque<(i8, i8)> = VecDeque::new();
        loop {
            for (point, octo) in octopus.iter_mut().filter(|(_, o)| **o != u8::MAX) {
                if *octo >= 10 {
                    flash_counter += 1;
                    *octo = u8::MAX;

                    NEIGHBOURS
                        .iter()
                        .map(|n| (point.0 + n.0, point.1 + n.1))
                        .for_each(|pt| {
                            cache.push_back(pt);
                        });
                }
            }

            while let Some(pt) = cache.pop_front() {
                octopus.entry(pt).and_modify(|e| *e = e.saturating_add(1));
            }

            if octopus
                .iter()
                .filter(|(_, o)| **o != u8::MAX)
                .all(|(_, o)| *o < 10)
            {
                break;
            }
        }

        octopus
            .iter_mut()
            .filter(|(_, o)| **o >= 10)
            .for_each(|(_, v)| *v = 0);

        // --
        if step == 100 {
            println!("Part one: {}", flash_counter);
        }
        if octopus.iter().all(|(_, o)| *o == 0) {
            println!("Part two: {}", step);
            break;
        }
    }
}

#[allow(dead_code)]
fn print(octopus: &HashMap<(i8, i8), u8>) {
    for row in 0..=octopus.keys().max_by_key(|(r, _)| r).unwrap().0 {
        for col in 0..=octopus.keys().max_by_key(|(_, c)| c).unwrap().1 {
            if let Some(octopus) = octopus.get(&(row, col)) {
                if *octopus == 0 {
                    print!("-");
                } else {
                    print!("{}", octopus);
                }
            }
        }
        println!();
    }
}
