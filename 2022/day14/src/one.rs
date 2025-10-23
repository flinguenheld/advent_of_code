use std::{cmp, collections::HashSet};

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let mut grid: HashSet<(i32, i32)> = HashSet::new();
    let mut sand: HashSet<(i32, i32)> = HashSet::new();

    for line in std::fs::read_to_string(path).unwrap().lines() {
        let mut previous = (-1, -1);
        for field in line.split(" -> ") {
            let (col, row) = field.split_once(',').unwrap();
            let row = row.parse::<i32>().unwrap();
            let col = col.parse::<i32>().unwrap();

            if previous == (-1, -1) {
            } else if previous.0 == row {
                for c in cmp::min(previous.1, col)..=cmp::max(previous.1, col) {
                    grid.insert((previous.0, c));
                }
            } else {
                for r in cmp::min(previous.0, row)..=cmp::max(previous.0, row) {
                    grid.insert((r, previous.1));
                }
            }
            previous = (row, col);
        }
    }

    let end_row = grid.iter().max_by_key(|(row, _)| row).unwrap().0;
    let mut drop = (0, 500);

    while drop.0 < end_row {
        if grid.contains(&(drop.0 + 1, drop.1)) || sand.contains(&(drop.0 + 1, drop.1)) {
            if !grid.contains(&(drop.0 + 1, drop.1 - 1))
                && !sand.contains(&(drop.0 + 1, drop.1 - 1))
            {
                drop = (drop.0 + 1, drop.1 - 1);
            } else if !grid.contains(&(drop.0 + 1, drop.1 + 1))
                && !sand.contains(&(drop.0 + 1, drop.1 + 1))
            {
                drop = (drop.0 + 1, drop.1 + 1);
            } else {
                sand.insert(drop);
                drop = (0, 500);
            }
            continue;
        }

        drop.0 += 1;
    }

    print(&grid, &sand);
    println!("Part one: {}", sand.len());
}

fn print(grid: &HashSet<(i32, i32)>, sand: &HashSet<(i32, i32)>) {
    // let start_row = grid.iter().min_by_key(|(row, _)| row).unwrap().0;
    let end_row = grid.iter().max_by_key(|(row, _)| row).unwrap().0;

    let start_col = grid.iter().min_by_key(|(_, col)| col).unwrap().1;
    let end_col = grid.iter().max_by_key(|(_, col)| col).unwrap().1;

    for row in 0..=end_row {
        for col in start_col..=end_col {
            if grid.contains(&(row, col)) {
                print!("#");
            } else if sand.contains(&(row, col)) {
                print!("o");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
