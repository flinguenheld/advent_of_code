use std::fs;

struct Slot {
    flower: char,
    done: bool,
}

#[derive(Debug)]
struct Count {
    flowers: u32,
    borders: u32,
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
                let mut count = Count {
                    flowers: 0,
                    borders: 0,
                };
                eat(
                    Point { row, col },
                    input[row][col].flower,
                    &mut input,
                    &mut count,
                );

                price += count.flowers * count.borders;
            }
        }
    }
    println!("Part one: {}", price);
    // display(&input);
}

#[rustfmt::skip]
fn eat(pos: Point, flower: char, input: &mut [Vec<Slot>], count: &mut Count) {
    if input[pos.row][pos.col].flower != flower {
        count.borders += 1;
    } else if !input[pos.row][pos.col].done {
        count.flowers += 1;
        input[pos.row][pos.col].done = true;

        if pos.row > 0 {
            eat( Point { row: pos.row - 1, col: pos.col, }, flower, input, count);
        } else { count.borders += 1; }

        if pos.row < input.len() - 1 {
            eat( Point { row: pos.row + 1, col: pos.col, }, flower, input, count);
        } else { count.borders += 1; }

        if pos.col > 0 {
            eat( Point { row: pos.row, col: pos.col - 1, }, flower, input, count);
        } else { count.borders += 1; }

        if pos.col < input[0].len() - 1 {
            eat( Point { row: pos.row, col: pos.col + 1, }, flower, input, count);
        } else { count.borders += 1; }
    }
}

fn display(input: &[Vec<Slot>]) {
    for row in input.iter() {
        for col in row.iter() {
            print!("{}", col.flower);
        }
        println!();
    }
}
