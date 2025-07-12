use colorized::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Dir {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone)]
struct Cart {
    pos: (i32, i32),
    dir: Dir,
    turn: usize,
    collided: bool,
}

impl Cart {
    fn up_position(&mut self) {
        self.pos = match self.dir {
            Dir::North => (self.pos.0 - 1, self.pos.1),
            Dir::East => (self.pos.0, self.pos.1 + 1),
            Dir::South => (self.pos.0 + 1, self.pos.1),
            Dir::West => (self.pos.0, self.pos.1 - 1),
        };
    }

    fn up_direction(&mut self, grid: &HashMap<(i32, i32), char>) {
        if let Some(&case) = grid.get(&self.pos) {
            if case == '\\' {
                self.dir = match self.dir {
                    Dir::North => Dir::West,
                    Dir::East => Dir::South,
                    Dir::South => Dir::East,
                    Dir::West => Dir::North,
                };
            } else if case == '/' {
                self.dir = match self.dir {
                    Dir::North => Dir::East,
                    Dir::East => Dir::North,
                    Dir::South => Dir::West,
                    Dir::West => Dir::South,
                };
            } else if case == '+' {
                if self.turn % 3 == 0 {
                    self.dir = match self.dir {
                        Dir::North => Dir::West,
                        Dir::East => Dir::North,
                        Dir::South => Dir::East,
                        Dir::West => Dir::South,
                    };
                } else if self.turn % 3 == 2 {
                    self.dir = match self.dir {
                        Dir::North => Dir::East,
                        Dir::East => Dir::South,
                        Dir::South => Dir::West,
                        Dir::West => Dir::North,
                    };
                }
                self.turn += 1;
            }
        }
    }
}

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";
    // let path = "input_example_2.txt";

    let input = std::fs::read_to_string(path).unwrap();

    let grid: HashMap<(i32, i32), char> = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.char_indices()
                .filter(|(_, c)| !c.is_whitespace())
                .map(move |(col, letter)| {
                    (
                        (row as i32, col as i32),
                        match letter {
                            '>' | '<' => '-',
                            'v' | '^' => '|',
                            l => l,
                        },
                    )
                })
        })
        .collect();

    let cart_base: Vec<Cart> = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.char_indices()
                .filter(|(_, c)| ['>', '<', '^', 'v'].contains(c))
                .map(move |(col, letter)| {
                    let dir = match letter {
                        '^' => Dir::North,
                        '>' => Dir::East,
                        'v' => Dir::South,
                        _ => Dir::West,
                    };

                    Cart {
                        pos: (row as i32, col as i32),
                        dir,
                        turn: 0,
                        collided: false,
                    }
                })
        })
        .collect();

    // print(&grid, &carts);
    let mut carts = cart_base.clone();

    'part_one: loop {
        for i in 0..carts.len() {
            carts[i].up_position();
            carts[i].up_direction(&grid);

            // Check here after EACH move -_-'
            if let Some(collision) = carts
                .iter()
                .find(|ca| carts.iter().filter(|c| c.pos == ca.pos).count() > 1)
            {
                println!("Part one: {},{}", collision.pos.1, collision.pos.0);
                break 'part_one;
            }
        }

        // print(&grid, &carts);
    }

    // --
    let mut carts = cart_base.clone();
    'part_two: loop {
        // Carts have to move in order, from top left to bottom right
        carts.retain(|ca| !ca.collided);
        carts.sort_by_key(|ca| ca.pos.0);
        carts.sort_by_key(|ca| ca.pos.1);

        if carts.len() == 1 {
            println!(
                "Part two: {},{}",
                carts.last().unwrap().pos.1,
                carts.last().unwrap().pos.0
            );
            break 'part_two;
        }

        for i in 0..carts.len() {
            carts[i].up_position();
            carts[i].up_direction(&grid);

            // Check & mark collisions
            let pos = carts[i].pos;
            if carts.iter().filter(|c| !c.collided && c.pos == pos).count() > 1 {
                carts
                    .iter_mut()
                    .filter(|c| c.pos == pos)
                    .for_each(|c| c.collided = true);
            }
        }

        // print(&grid, &carts);
    }
}

#[allow(dead_code)]
fn print(grid: &HashMap<(i32, i32), char>, carts: &[Cart]) {
    let width = grid.keys().max_by_key(|(_, c)| c).unwrap().1;
    let height = grid.keys().max_by_key(|(r, _)| r).unwrap().0;

    for row in 0..height + 1 {
        for col in 0..width + 1 {
            if let Some(cart) = carts.iter().find(|c| !c.collided && c.pos == (row, col)) {
                print!(
                    "{}{}",
                    Colors::RedFg.value(),
                    match cart.dir {
                        Dir::North => '^',
                        Dir::East => '>',
                        Dir::South => 'v',
                        Dir::West => '<',
                    },
                );
            } else if let Some(c) = grid.get(&(row, col)) {
                print!("{}{}", Colors::WhiteFg.value(), c);
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
