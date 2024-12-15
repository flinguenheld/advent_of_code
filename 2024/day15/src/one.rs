use colored::Colorize;
use std::fs;

#[derive(Clone)]
struct Point {
    row: usize,
    col: usize,
}
impl Point {
    fn next(&self, command: char, map: &[Vec<char>]) -> Option<Point> {
        let mut row = self.row;
        let mut col = self.col;

        match command {
            '^' => row -= 1,
            '>' => col += 1,
            'v' => row += 1,
            _ => col -= 1,
        }

        match map[row][col] {
            '#' => None,
            _ => Some(Point { row, col }),
        }
    }
}

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";
    // let path = "input_example_small.txt";

    let input = fs::read_to_string(path).unwrap();

    let mut map: Vec<Vec<char>> = Vec::new();
    let mut commands: Vec<char> = Vec::new();
    let mut first = true;

    for line in input.lines() {
        if line.is_empty() {
            first = false;
        } else if first {
            map.push(line.chars().collect());
        } else {
            commands.append(&mut line.chars().collect());
        }
    }

    let (nb_row, nb_col) = (map.len(), map[0].len());

    // Where is my robot ?
    let mut robot = Point { row: 0, col: 0 };
    (0..nb_row).for_each(|row| {
        for col in 0..nb_col {
            if map[row][col] == '@' {
                robot.row = row;
                robot.col = col;
                break;
            }
        }
    });

    // Part one --
    display(&map);

    for command in commands.iter() {
        if let Some(next) = robot.next(*command, &map) {
            if map[next.row][next.col] == 'O' {
                if move_this_line(&next, *command, &mut map) {
                    map[robot.row][robot.col] = '.';
                    robot = next.clone();
                    map[robot.row][robot.col] = '@';
                }
            } else {
                map[robot.row][robot.col] = '.';
                robot = next.clone();
                map[robot.row][robot.col] = '@';
            }
        }
    }

    display(&map);

    let total = (0..nb_row).fold(0, |acc, row| {
        acc + (0..nb_col).fold(0, |acc2, col| {
            if map[row][col] == 'O' {
                acc2 + 100 * row + col
            } else {
                acc2
            }
        })
    });

    println!("Part one: {}", total);

    // dbg!(&input);
    // dbg!(&commands);
}

fn move_this_line(from: &Point, command: char, map: &mut [Vec<char>]) -> bool {
    // Look for the next dot
    let mut current = from.clone();
    while let Some(next) = current.next(command, map) {
        if map[next.row][next.col] == '.' {
            // Set this as O and remove the first one
            map[next.row][next.col] = 'O';
            map[from.row][from.col] = '.';
            return true;
        } else {
            current = next;
        }
    }
    false
}

fn display(map: &[Vec<char>]) {
    for row in map.iter() {
        for col in row.iter() {
            print!(
                "{}",
                match col {
                    '#' => "#".blue(),
                    'O' => "0".yellow(),
                    '@' => "@".red(),
                    _ => ".".bright_black(),
                }
            );
        }
        println!();
    }
}
