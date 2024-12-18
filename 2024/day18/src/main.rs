use colored::Colorize;

// const PATH: &str = "input.txt";
// const MAP_SIZE: usize = 70;
// const NB_START: usize = 1024;

const PATH: &str = "input_example.txt";
const MAP_SIZE: usize = 6;
const NB_START: usize = 12;

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
const NON_VISITED: i32 = -1;
const CORRUPTED: i32 = -2;

use std::fs;

fn main() {
    let mut map = vec![vec![NON_VISITED; MAP_SIZE + 1]; MAP_SIZE + 1];

    let corrupted_coordinates: Vec<(usize, usize)> = fs::read_to_string(PATH)
        .unwrap()
        .lines()
        .map(|line| {
            let (col, row) = line.split_once(',').unwrap();
            (row.parse::<usize>().unwrap(), col.parse::<usize>().unwrap())
        })
        .collect();

    // Init --
    map[0][0] = 0;
    for coordinate in corrupted_coordinates.iter().take(NB_START) {
        map[coordinate.0][coordinate.1] = CORRUPTED;
    }

    // Part one --
    let (_, val, mut map_ok) = bfs(map.clone());
    trace(&mut map_ok, (MAP_SIZE, MAP_SIZE), val);
    display(&map_ok);

    println!("Part one: {}", val);

    // Part two --
    for coordinate in corrupted_coordinates.iter().skip(NB_START) {
        map[coordinate.0][coordinate.1] = CORRUPTED;

        let (ok, val, mut map_nok) = bfs(map.clone());
        if ok {
            trace(&mut map_nok, (coordinate.0, coordinate.1), val - 1);
            display(&map_nok);

            println!("Part two: {},{}", coordinate.1, coordinate.0);
            break;
        }
    }
}

fn bfs(mut map: Vec<Vec<i32>>) -> (bool, i32, Vec<Vec<i32>>) {
    let mut current_value = 0;

    let mut is_blocked;
    'aaa: loop {
        is_blocked = true;
        for row in 0..=MAP_SIZE {
            for col in 0..=MAP_SIZE {
                if map[row][col] == current_value {
                    for direction in DIRECTIONS.iter() {
                        if let Some((next_row, next_col)) = is_direction_valid(direction, row, col)
                        {
                            is_blocked = false;

                            if next_row == MAP_SIZE && next_col == MAP_SIZE {
                                // Exit reached
                                break 'aaa;
                            } else if map[next_row][next_col] == NON_VISITED {
                                map[next_row][next_col] = current_value + 1;
                            }
                        }
                    }
                }
            }
        }
        if is_blocked {
            break 'aaa;
        }
        current_value += 1;
    }

    (is_blocked, current_value + 1, map)
}

fn is_direction_valid(direction: &(i32, i32), row: usize, col: usize) -> Option<(usize, usize)> {
    let row = row as i32 + direction.0;
    let col = col as i32 + direction.1;

    if row >= 0 && row <= MAP_SIZE as i32 && col >= 0 && col <= MAP_SIZE as i32 {
        Some((row as usize, col as usize))
    } else {
        None
    }
}

fn trace(map: &mut [Vec<i32>], from: (usize, usize), current_value: i32) {
    for direction in DIRECTIONS.iter() {
        if let Some((next_row, next_col)) = is_direction_valid(direction, from.0, from.1) {
            if next_row == 0 && next_col == 0 {
                // println!("OK");
            } else if map[next_row][next_col] == current_value - 1 {
                map[next_row][next_col] = 0;
                trace(map, (next_row, next_col), current_value - 1);
                break;
            }
        }
    }
}

fn display(map: &[Vec<i32>]) {
    for row in map.iter() {
        for col in row.iter() {
            print!(
                "{}",
                match col {
                    &NON_VISITED => ".".bright_black(),
                    &CORRUPTED => "#".red(),
                    0 => "o".blue(),
                    _ => ".".bright_black(),
                }
            );
        }
        println!();
    }
}
