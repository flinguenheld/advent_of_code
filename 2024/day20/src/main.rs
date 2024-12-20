use std::fs;

struct Point {
    row: i32,
    col: i32,
}

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let input = fs::read_to_string(path).unwrap();

    let mut path = Vec::new();
    let mut map = Vec::new();

    for (row, line) in input.lines().enumerate() {
        map.push(Vec::new());
        for (col, case) in line.chars().enumerate() {
            map.last_mut().unwrap().push(case);
            if case == 'S' {
                path.push(Point {
                    row: row as i32,
                    col: col as i32,
                });
            }
        }
    }

    walk(&mut map, &mut path);

    println!("Part one: {}", count(&path, 2, 100));
    println!("Part two: {}", count(&path, 20, 100));
}

// For each points, read all the next ones and check if a cheat is possible
fn count(path: &[Point], radius: i32, cheat_mini: i32) -> i32 {
    path.iter().enumerate().fold(0, |acc, (i, start)| {
        acc + path
            .iter()
            .enumerate()
            .skip(i + 1)
            .fold(0, |acc2, (j, end)| {
                let distance = (end.row - start.row).abs() + (end.col - start.col).abs();
                if distance <= radius {
                    let cheat = (j as i32 - i as i32).abs() - distance;
                    if cheat >= cheat_mini {
                        acc2 + 1
                    } else {
                        acc2
                    }
                } else {
                    acc2
                }
            })
    })
}

// Explore the path and save the points
fn walk(map: &mut Vec<Vec<char>>, steps: &mut Vec<Point>) {
    if let Some(last) = steps.last() {
        for (r, c) in [(-1, 0), (0, 1), (1, 0), (0, -1)].iter() {
            let row = (last.row + r) as usize;
            let col = (last.col + c) as usize;

            if map[row][col] == '.' || map[row][col] == 'E' {
                map[row][col] = 'o';
                steps.push(Point {
                    row: row as i32,
                    col: col as i32,
                });
                walk(map, steps);
                break;
            }
        }
    }
}
