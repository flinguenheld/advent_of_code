use std::fs;

struct Slot {
    flower: char,
    done: bool,
}

struct Point {
    row: usize,
    col: usize,
}

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let input = fs::read_to_string(path).unwrap();
    let mut input: Vec<Vec<Slot>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Slot {
                    flower: c,
                    done: false,
                })
                .collect()
        })
        .collect();

    let mut price = 0;
    for row in 0..input.len() {
        for col in 0..input[0].len() {
            if !input[row][col].done {
                let mut temp = vec![vec!['.'; input[0].len()]; input.len()];
                let mut count = 0;
                let mut borders = 0;
                eat(
                    Point { row, col },
                    input[row][col].flower,
                    &mut input,
                    &mut count,
                    &mut temp,
                );

                // display(&temp);
                borders += count_border_top(&temp);
                temp.reverse();
                borders += count_border_top(&temp);

                borders += count_border_left(&temp);
                temp.iter_mut().for_each(|line| line.reverse());
                borders += count_border_left(&temp);

                price += borders * count;
            }
        }
    }
    println!("Part two: {}", price);
    // display(&input);
}

fn count_border_top(temp: &[Vec<char>]) -> u32 {
    let mut total = 0;
    for row in 0..temp.len() {
        let mut cont = false;
        for col in 0..temp[0].len() {
            let top = if row == 0 { '0' } else { temp[row - 1][col] };

            let current = if top != '#' && temp[row][col] == '#' {
                '#'
            } else {
                '\0'
            };

            if top != '#' && current == '#' {
                cont = true;
            }

            if cont && (current != '#' || col == temp[0].len() - 1) {
                total += 1;
                cont = false;
            }
        }
    }
    total
}

fn count_border_left(temp: &[Vec<char>]) -> u32 {
    let mut total = 0;
    for col in 0..temp[0].len() {
        let mut continuous = false;
        for row in 0..temp.len() {
            let left = if col == 0 { '0' } else { temp[row][col - 1] };

            let current = if left != '#' && temp[row][col] == '#' {
                '#'
            } else {
                '\0'
            };

            if left != '#' && current == '#' {
                continuous = true;
            }

            if continuous && (current != '#' || row == temp.len() - 1) {
                total += 1;
                continuous = false;
            }
        }
    }

    total
}

#[rustfmt::skip]
fn eat(pos: Point, flower: char, input: &mut [Vec<Slot>], count: &mut u32, temp: &mut [Vec<char>]) {
    if input[pos.row][pos.col].flower != flower {
        // count.borders += 1;
    } else if !input[pos.row][pos.col].done {
        *count += 1;
        input[pos.row][pos.col].done = true;
        temp[pos.row][pos.col]= '#';

        if pos.row > 0 {
            eat( Point { row: pos.row - 1, col: pos.col, }, flower, input, count, temp);
        }

        if pos.row < input.len() - 1 {
            eat( Point { row: pos.row + 1, col: pos.col, }, flower, input, count, temp);
        }

        if pos.col > 0 {
            eat( Point { row: pos.row, col: pos.col - 1, }, flower, input, count, temp);
        }

        if pos.col < input[0].len() - 1 {
            eat( Point { row: pos.row, col: pos.col + 1, }, flower, input, count, temp);
        }
    }
}

fn display(input: &[Vec<char>]) {
    for row in input.iter() {
        for col in row.iter() {
            print!("{}", col);
        }
        println!();
    }
}
