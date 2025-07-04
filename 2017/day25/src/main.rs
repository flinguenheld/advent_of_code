use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct BluePrint {
    write: bool,
    movement: i32,
    next: char,
}

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let input = std::fs::read_to_string(path).unwrap();
    let input: Vec<&str> = input.lines().collect();

    let mut bp = input[0][15..16].chars().next().unwrap();
    let steps = input[1]
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    let blueprints: HashMap<char, (BluePrint, BluePrint)> = input[3..]
        .windows(9)
        .step_by(10)
        .map(|group| {
            let key = group[0][9..10].chars().next().unwrap();

            (key, (extract(&group[6..9]), extract(&group[2..5])))
        })
        .collect();

    let mut tape: HashSet<i32> = HashSet::new();
    let mut cursor = 0;
    for _ in 0..steps {
        if let Some(blue_print) = blueprints.get(&bp) {
            if tape.contains(&cursor) {
                if !blue_print.0.write {
                    tape.remove(&cursor);
                }
                cursor += blue_print.0.movement;
                bp = blue_print.0.next;
            } else {
                if blue_print.1.write {
                    tape.insert(cursor);
                }
                cursor += blue_print.1.movement;
                bp = blue_print.1.next;
            }
        }
    }

    println!("Final part: {}", tape.len());
}

fn extract(window: &[&str]) -> BluePrint {
    BluePrint {
        write: window[0].contains('1'),
        movement: if window[1].contains("right") { 1 } else { -1 },
        next: window[2][26..27].chars().next().unwrap(),
    }
}
