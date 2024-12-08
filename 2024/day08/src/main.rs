use colored::Colorize;
use std::fs;

struct Layer {
    antenna: char,
    antinodes: bool,
}

#[derive(Clone, Copy)]
struct Point {
    row: usize,
    col: usize,
}

impl Point {
    fn next(&self, input: &[Vec<Layer>]) -> Option<Point> {
        let mut row = self.row;
        let mut col = self.col + 1;

        if col >= input[0].len() {
            row += 1;
            col = 0;
        }

        if row < input.len() {
            Some(Point { row, col })
        } else {
            None
        }
    }
    fn up(&self, input: &[Vec<Layer>], dis_row: i32, dis_col: i32) -> Option<Point> {
        let new_row = self.row as i32 + dis_row;
        let new_col = self.col as i32 + dis_col;

        if new_row >= 0
            && new_col >= 0
            && new_row < input.len() as i32
            && new_col < input[0].len() as i32
        {
            Some(Point {
                row: new_row as usize,
                col: new_col as usize,
            })
        } else {
            None
        }
    }
}

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let input = fs::read_to_string(path).unwrap();

    let mut input: Vec<Vec<Layer>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Layer {
                    antenna: c,
                    antinodes: false,
                })
                .collect()
        })
        .collect();

    // Part one --
    // 1- Find all antennas
    let mut current_pos = Point { row: 0, col: 0 };
    while let Some(mother) = next_antenna(&input, '*', &current_pos) {
        let kind = input[mother.row][mother.col].antenna;

        // 2- Find all the next antennas from this one
        current_pos = mother;
        while let Some(child) = next_antenna(&input, kind, &current_pos) {
            // 3 Measure the distance
            let distance = (
                child.row as i32 - mother.row as i32,
                child.col as i32 - mother.col as i32,
            );

            // 4 Put antinodes in both directions if possible
            if let Some(antinode_north) = mother.up(&input, -distance.0, -distance.1) {
                input[antinode_north.row][antinode_north.col].antinodes = true;
            }
            if let Some(antinode_south) = child.up(&input, distance.0, distance.1) {
                input[antinode_south.row][antinode_south.col].antinodes = true;
            }

            current_pos = child;
        }
        current_pos = mother;
    }
    display(&input);
    println!(
        "Part one: {}",
        input.iter().flatten().filter(|c| c.antinodes).count()
    );

    // Part two --
    input.iter_mut().flatten().for_each(|c| c.antinodes = false);

    // 1- Find all antennas
    let mut current_pos = Point { row: 0, col: 0 };
    while let Some(mother) = next_antenna(&input, '*', &current_pos) {
        let kind = input[mother.row][mother.col].antenna;

        // 2- Find all the next antennas from this one
        current_pos = mother;
        while let Some(child) = next_antenna(&input, kind, &current_pos) {
            // 3- Measure the distance
            let distance = (
                child.row as i32 - mother.row as i32,
                child.col as i32 - mother.col as i32,
            );

            // 4- Put antinodes in both directions until it is possible
            let mut starting_point = mother;
            while let Some(antinode_north) = starting_point.up(&input, -distance.0, -distance.1) {
                input[antinode_north.row][antinode_north.col].antinodes = true;
                starting_point = antinode_north;
            }
            starting_point = child;
            while let Some(antinode_south) = starting_point.up(&input, distance.0, distance.1) {
                input[antinode_south.row][antinode_south.col].antinodes = true;
                starting_point = antinode_south;
            }

            // 5- Set mother & child also as antinodes
            input[child.row][child.col].antinodes = true;
            input[mother.row][mother.col].antinodes = true;

            current_pos = child;
        }
        current_pos = mother;
    }

    display(&input);
    println!(
        "Part two: {}",
        input.iter().flatten().filter(|c| c.antinodes).count()
    );
}

fn next_antenna(input: &[Vec<Layer>], kind: char, start: &Point) -> Option<Point> {
    let mut start = *start;
    while let Some(next_one) = start.next(input) {
        let case = input[next_one.row][next_one.col].antenna;
        if case == kind || (kind == '*' && case != '.') {
            return Some(next_one);
        }
        start = next_one;
    }

    None
}

fn display(input: &[Vec<Layer>]) {
    for row in input.iter() {
        for col in row.iter() {
            print!(
                "{}",
                match col.antinodes {
                    true => {
                        match col.antenna {
                            '.' => '#'.to_string().yellow(),
                            _ => col.antenna.to_string().red(),
                        }
                    }
                    false => {
                        match col.antenna {
                            '.' => '.'.to_string().bright_black(),
                            _ => col.antenna.to_string().blue(),
                        }
                    }
                }
            );
        }
        println!();
    }
}
