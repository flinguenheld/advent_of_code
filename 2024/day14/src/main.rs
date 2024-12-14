use std::fs;

const ROWS: usize = 103;
const COLUMNS: usize = 101;
const PATH: &str = "input.txt";

struct Pouet {
    pos_row: i32,
    pos_col: i32,

    vel_row: i32,
    vel_col: i32,
}

#[rustfmt::skip]
fn main() {
    let input = fs::read_to_string(PATH).unwrap();

    let mut commands = Vec::new();
    for line in input.lines() {
        let (coordinates, speed) = line.split_once(' ').unwrap();
        let (position_col, position_row) = coordinates.split_once(',').unwrap();
        let (velocity_col, velocity_row) = speed.split_once(',').unwrap();

        let pos_row = convert(position_row);
        let pos_col = convert(position_col);

        let vel_row = convert(velocity_row);
        let vel_col = convert(velocity_col);

        commands.push(Pouet {
            pos_row,
            pos_col,
            vel_row,
            vel_col,
        });
    }

    // Part one --
    let map = move_it(100, &commands);

    let mut quadrant = [0, 0, 0, 0];
    for (r, row) in map.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            match r.cmp(&(ROWS / 2)) {
                std::cmp::Ordering::Greater => match c.cmp(&(COLUMNS / 2)) {
                    std::cmp::Ordering::Greater => { quadrant[0] += col; }
                    std::cmp::Ordering::Less => { quadrant[1] += col; }
                    _ => {}
                },
                std::cmp::Ordering::Less => match c.cmp(&(COLUMNS / 2)) {
                    std::cmp::Ordering::Greater => { quadrant[2] += col; }
                    std::cmp::Ordering::Less => { quadrant[3] += col; }
                    _ => {}
                },
                _ => {}
            }
        }
    }
    // display(&map);
    println!("Part one: {}", quadrant.iter().product::<u32>());

    // Part two --
    let mut second = 0;
    'tree: loop {
        second += 1;
        let map = move_it(second, &commands);

        for line in map.iter() {
            for win in line.windows(15).step_by(15) {
                if win.iter().all(|v| *v > 0) {
                    // display(&map);
                    println!("Part two: {}", second);
                    break 'tree;
                }
            }
        }
    }
}

fn move_it(nb_moves: i32, commands: &[Pouet]) -> [[u32; COLUMNS]; ROWS] {
    let mut map: [[u32; COLUMNS]; ROWS] = [[0; COLUMNS]; ROWS];
    for command in commands.iter() {
        let mut row = command.pos_row;
        let mut col = command.pos_col;

        let vel_row = command.vel_row;
        let vel_col = command.vel_col;

        row += vel_row * nb_moves;
        col += vel_col * nb_moves;

        row = row.rem_euclid(ROWS as i32);
        col = col.rem_euclid(COLUMNS as i32);

        map[row as usize][col as usize] += 1;
    }
    map
}

fn convert(txt: &str) -> i32 {
    let from: String = txt
        .chars()
        .filter(|c| c.is_ascii_digit() || *c == '-')
        .collect();

    from.parse::<i32>().unwrap()
}

fn display(map: &[[u32; COLUMNS]; ROWS]) {
    (0..ROWS).for_each(|row| {
        for col in 0..COLUMNS {
            match map[row][col] {
                0 => {
                    print!(".");
                }
                n if n < 10 => {
                    print!("{}", n);
                }
                _ => {
                    print!("X");
                }
            }
        }
        println!();
    });
}
