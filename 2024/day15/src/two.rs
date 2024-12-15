use colored::Colorize;
use std::fs;

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Eq)]
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
            let mut current_line = Vec::new();
            for c in line.chars() {
                match c {
                    'O' => {
                        current_line.push('[');
                        current_line.push(']');
                    }
                    '.' => {
                        current_line.push('.');
                        current_line.push('.');
                    }
                    '#' => {
                        current_line.push('#');
                        current_line.push('#');
                    }
                    _ => {
                        current_line.push('@');
                        current_line.push('.');
                    }
                }
            }

            map.push(current_line);
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

    // Part two --
    display(&map);

    'com: for command in commands.iter() {
        if let Some(next) = robot.next(*command, &map) {
            let next_value = map[next.row][next.col];

            // Simple move --
            if next_value == '.' {
                robot = move_robot(robot, next, &mut map);

            // Left right --
            } else if *command == '<' || *command == '>' {
                if (map[next.row][next.col] == ']' || map[next.row][next.col] == '[')
                    && move_horizontal(&next, *command, &mut map)
                {
                    robot = move_robot(robot, next, &mut map);
                }

            // Up down --
            } else {
                let mut boxes: Vec<Point> = Vec::new();
                list_boxes(&robot, *command, &mut boxes, &map);

                // Is it possible to move the entire block ?
                for point in boxes.iter() {
                    if point.next(*command, &map).is_none() {
                        continue 'com;
                    }
                }

                // Do it !
                match command {
                    'v' => boxes.sort_by(|a, b| a.row.cmp(&b.row)),
                    _ => boxes.sort_by(|a, b| b.row.cmp(&a.row)),
                }

                for point in boxes.iter().rev() {
                    let value = map[point.row][point.col];
                    map[point.row][point.col] = '.';

                    let next = point.next(*command, &map).unwrap();
                    map[next.row][next.col] = value;
                }

                robot = move_robot(robot, next, &mut map);
            }
        }
        // println!("--------------------------------------------");
        // println!("Command: {}", command);
        // display(&map);
    }

    let total = (0..nb_row).fold(0, |acc, row| {
        acc + (0..nb_col).fold(0, |acc2, col| {
            if map[row][col] == '[' {
                acc2 + 100 * row + col
            } else {
                acc2
            }
        })
    });

    display(&map);
    println!("Part two: {}", total);
}

fn move_robot(robot: Point, next: Point, map: &mut [Vec<char>]) -> Point {
    map[robot.row][robot.col] = '.';
    map[next.row][next.col] = '@';
    next.clone()
}

// Add all connected boxes in 'points'
fn list_boxes(from: &Point, command: char, points: &mut Vec<Point>, map: &[Vec<char>]) {
    if let Some(next_position) = from.next(command, map) {
        let value_next = map[next_position.row][next_position.col];

        if value_next == '[' {
            let next_right = Point {
                row: next_position.row,
                col: next_position.col + 1,
            };

            list_boxes(&next_position, command, points, map);
            list_boxes(&next_right, command, points, map);

            add_without_duplicates(points, &next_right);
            add_without_duplicates(points, &next_position);
        } else if value_next == ']' {
            let next_left = Point {
                row: next_position.row,
                col: next_position.col - 1,
            };

            list_boxes(&next_position, command, points, map);
            list_boxes(&next_left, command, points, map);

            add_without_duplicates(points, &next_left);
            add_without_duplicates(points, &next_position);
        }
    }
}

fn add_without_duplicates(points: &mut Vec<Point>, new: &Point) {
    if !points.contains(new) {
        points.push(new.clone());
    }
}

fn move_horizontal(from: &Point, command: char, map: &mut [Vec<char>]) -> bool {
    // Look for the next dot
    let mut current = from.clone();
    let mut todo = Vec::new();
    while let Some(next) = current.next(command, map) {
        todo.push(next.clone());
        if map[next.row][next.col] == '.' {
            // Set this as O and remove the first one
            todo.iter().enumerate().for_each(|(i, p)| {
                map[p.row][p.col] = match (i % 2, command) {
                    (0, '>') => '[',
                    (_, '>') => ']',
                    (0, '<') => ']',
                    _ => '[',
                };
            });
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
                    '[' => "[".yellow(),
                    ']' => "]".yellow(),
                    '@' => "@".red(),
                    _ => ".".bright_black(),
                }
            );
        }
        println!();
    }
}
