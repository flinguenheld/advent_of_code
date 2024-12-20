use colored::Colorize;
use std::fs;

const WALL: i32 = -3;
const START: i32 = -2;
const END: i32 = -1;

const CHEAT_MINI: i32 = 100;

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let input = fs::read_to_string(path).unwrap();

    let mut map: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => WALL,
                    'S' => START,
                    // 'E' => END,
                    _ => 0,
                })
                .collect()
        })
        .collect();

    let mut start = (0, 0);
    'aaa: for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row][col] == START {
                start = (row, col);
                break 'aaa;
            }
        }
    }

    let steps = walk(&mut map, start, 0);
    println!("Steps: {}", steps);

    let horizontal_cheats = map.iter().fold(0, |acc, row| {
        acc + row
            .windows(3)
            .filter(|win| win[0] != WALL && win[1] == WALL && win[2] != WALL)
            .map(|win| (win[2] - win[0]).abs() - 2)
            .filter(|cheat| *cheat >= CHEAT_MINI)
            .count()
    });

    let mut vertical_cheats = 0;
    for row in 1..map.len() - 1 {
        for col in 0..map[0].len() {
            if map[row - 1][col] != WALL && map[row][col] == WALL && map[row + 1][col] != WALL {
                let cheat = (map[row - 1][col] - map[row + 1][col]).abs() - 2;
                println!("saves {} !!", cheat);
                if cheat >= CHEAT_MINI {
                    vertical_cheats += 1;
                }
            }
        }
    }

    display(&map);

    println!("Cheats: {}", horizontal_cheats);
    println!("Cheats: {}", vertical_cheats);

    println!("Part one: {}", horizontal_cheats + vertical_cheats);
}

fn walk(map: &mut Vec<Vec<i32>>, position: (usize, usize), value: i32) -> i32 {
    map[position.0][position.1] = value;
    for (r, c) in [(-1, 0), (0, 1), (1, 0), (0, -1)].iter() {
        let row = (position.0 as i32 + r) as usize;
        let col = (position.1 as i32 + c) as usize;
        if map[row][col] == 0 {
            return walk(map, (row, col), value + 1);
        }
    }
    value
}

fn display(map: &[Vec<i32>]) {
    for row in map.iter() {
        for col in row.iter() {
            print!(
                "{}",
                match *col {
                    WALL => "█".bright_black(),
                    START => "S".red(),
                    END => "E".red(),
                    // 0 => ".".bright_black(),
                    _ => format!("{}", col % 10).yellow(),
                }
            );
        }
        println!();
    }
}
