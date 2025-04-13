use std::{collections::HashSet, fs};

// const TAIL_LENGTH: usize = 2;
const TAIL_LENGTH: usize = 10;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Point {
    row: isize,
    col: isize,
}

fn main() {
    // let path = "input_example.txt";
    // let path = "input_example_2.txt";
    let path = "input.txt";

    let mut long_tail = [Point { row: 0, col: 0 }; TAIL_LENGTH];

    let mut visited: HashSet<Point> = HashSet::new();
    let input = fs::read_to_string(path).unwrap();

    for command in input.lines() {
        if let Some((direction, steps)) = command.split_once(' ') {
            if let Ok(steps) = steps.parse::<u8>() {
                for _ in 0..steps {
                    match direction {
                        "U" => long_tail.first_mut().unwrap().row -= 1,
                        "D" => long_tail.first_mut().unwrap().row += 1,
                        "L" => long_tail.first_mut().unwrap().col -= 1,
                        _ => long_tail.first_mut().unwrap().col += 1,
                    }

                    for i in 1..TAIL_LENGTH {
                        let distance = (
                            long_tail[i - 1].row - long_tail[i].row,
                            long_tail[i - 1].col - long_tail[i].col,
                        );
                        println!("distance: {}  {}", distance.0, distance.1);

                        if distance.0.abs() > 1 || distance.1.abs() > 1 {
                            long_tail[i].row += distance.0.signum();
                            long_tail[i].col += distance.1.signum();
                        }
                    }

                    visited.insert(*long_tail.last().unwrap());
                    // display(&visited);
                }
            }
        }
    }

    display(&visited);
    println!("Visited: {}", visited.len());
}

fn display(visited: &HashSet<Point>) {
    let min_row = visited.iter().map(|p| p.row).min().unwrap();
    let min_col = visited.iter().map(|p| p.col).min().unwrap();

    let width =
        visited.iter().map(|p| p.col).max().unwrap() - visited.iter().map(|p| p.col).min().unwrap();
    let height =
        visited.iter().map(|p| p.row).max().unwrap() - visited.iter().map(|p| p.row).min().unwrap();
    let mut grid = vec![vec!['.'; width as usize + 1]; height as usize + 1];

    for point in visited.iter() {
        grid[(point.row - min_row) as usize][(point.col - min_col) as usize] = '#';
    }

    for r in grid.iter() {
        for c in r.iter() {
            print!("{}", c);
        }
        println!();
    }
}
