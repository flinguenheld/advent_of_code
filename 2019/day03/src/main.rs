use std::collections::HashMap;

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let input = std::fs::read_to_string(path).unwrap();
    let input: Vec<&str> = input.lines().collect();

    let a: HashMap<(i32, i32), u32> = fill(input[0]);
    let b: HashMap<(i32, i32), u32> = fill(input[1]);

    // print(&a);
    // print(&b);

    // Coordinates + distances
    let crossed: Vec<((i32, i32), i32)> = a
        .keys()
        .filter(|k| b.contains_key(k))
        .map(|(r, c)| ((*r, *c), r.abs() + c.abs()))
        .collect();

    println!(
        "Part one: {}",
        crossed.iter().min_by_key(|(_, v)| v).unwrap().1
    );

    // --
    let steps: Vec<u32> = crossed
        .iter()
        .map(|pt| a.get(&pt.0).unwrap() + b.get(&pt.0).unwrap())
        .collect();

    println!("Part two: {}", steps.iter().min().unwrap());
}

// Get all points with their minimal step number
fn fill(movements: &str) -> HashMap<(i32, i32), u32> {
    let mut points: HashMap<(i32, i32), u32> = HashMap::new();
    let mut current = (0, 0);
    let mut count = 0;

    for movement in movements.split(',') {
        let m = movement.chars().next().unwrap();
        let nb = movement
            .chars()
            .skip(1)
            .collect::<String>()
            .parse::<u32>()
            .unwrap();

        for _ in 0..nb {
            count += 1;
            match m {
                'U' => current.0 -= 1,
                'D' => current.0 += 1,
                'L' => current.1 -= 1,
                _ => current.1 += 1,
            }

            points.entry(current).or_insert(count);
        }
    }

    points
}

#[allow(dead_code)]
fn print(points: &HashMap<(i32, i32), u32>) {
    let min_row = points.keys().min_by_key(|(row, _)| row).unwrap().0;
    let max_row = points.keys().max_by_key(|(row, _)| row).unwrap().0;

    let min_col = points.keys().min_by_key(|(_, col)| col).unwrap().1;
    let max_col = points.keys().max_by_key(|(_, col)| col).unwrap().1;

    for row in min_row..=max_row {
        for col in min_col..=max_col {
            if points.contains_key(&(row, col)) {
                print!("o");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
