use std::collections::HashMap;

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let mut current: (i32, i32, i32) = (0, 0, 0);
    let mut mem: HashMap<(i32, i32, i32), i32> = HashMap::new();

    let input = std::fs::read_to_string(path).unwrap();
    for dir in input[..input.len() - 1].split(',') {
        current = step(current, dir);
        distance(&current, &mut mem);
    }

    println!("Part one: {}", distance(&current, &mut mem));
    println!("Part two: {}", mem.values().max().unwrap());
}

// From each cases in the north, collect all cases with the same distance from 0,0
// Do it until it has found "current"
// Memorise all coordinates with their distance
fn distance(current: &(i32, i32, i32), mem: &mut HashMap<(i32, i32, i32), i32>) -> i32 {
    if let Some(val) = mem.get(current) {
        *val
    } else if *current != (0, 0, 0) {
        let mut points = [
            ((0, 0, 0), "n", "se"),
            ((0, 0, 0), "ne", "s"),
            ((0, 0, 0), "se", "sw"),
            ((0, 0, 0), "s", "nw"),
            ((0, 0, 0), "sw", "n"),
            ((0, 0, 0), "nw", "ne"),
            ((0, 0, 0), "n", "prout"),
        ];

        // Restart from the last highest distance
        let mut d = *mem.values().max().unwrap_or(&0);
        for _ in 0..d {
            points.iter_mut().for_each(|p| p.0 = step(p.0, p.1));
        }

        let mut stop = false;
        while !stop {
            d += 1;
            points.iter_mut().for_each(|(p, dir, _)| *p = step(*p, dir));

            for win in points.windows(2) {
                if get_row(win[0].0, win[1].0, win[0].2, *current, mem, d) {
                    // Do not end here, collect oll coordinates with this value.
                    stop = true;
                }
            }
        }
        d
    } else {
        0
    }
}

// Add all coordinations in mem
fn get_row(
    from: (i32, i32, i32),
    to: (i32, i32, i32),
    dir: &str,
    target: (i32, i32, i32),
    mem: &mut HashMap<(i32, i32, i32), i32>,
    distance: i32,
) -> bool {
    let mut found = false;
    let mut from = from;
    mem.insert(from, distance);

    while from != to {
        from = step(from, dir);
        mem.insert(from, distance);

        if from == target {
            found = true;
        }
    }
    found
}

fn step(mut current: (i32, i32, i32), direction: &str) -> (i32, i32, i32) {
    if direction == "n" {
        current.2 += 1;
    } else if direction == "s" {
        current.2 -= 1;
    } else if current.0 == 0 {
        current.0 = 1;
        match direction {
            "ne" => {}
            "se" => current.2 -= 1,
            "sw" => {
                current.1 -= 1;
                current.2 -= 1;
            }
            "nw" => current.1 -= 1,
            _ => {}
        }
    } else {
        current.0 = 0;
        match direction {
            "ne" => {
                current.1 += 1;
                current.2 += 1;
            }
            "se" => current.1 += 1,
            "sw" => {}
            "nw" => current.2 += 1,
            _ => {}
        }
    }
    current
}
