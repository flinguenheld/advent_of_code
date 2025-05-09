use std::{collections::VecDeque, fs};

#[derive(PartialEq, PartialOrd, Clone, Copy)]
enum Elem {
    Value(u64),
    Plus,
    Mul,
    Open,
    Close,
}

impl Elem {
    fn get(&self) -> Option<u64> {
        if let Elem::Value(value) = self {
            Some(*value)
        } else {
            None
        }
    }
}

fn main() {
    // let path = "input_example.txt";
    let path = "input.txt";

    let mut part_one = 0;
    let mut part_two = 0;
    for line in fs::read_to_string(path).unwrap().lines() {
        let line = line
            .replace(")", " )")
            .split_inclusive(|c| [' ', '(', ')'].contains(&c))
            .filter(|txt| *txt != " ")
            .map(|txt| match txt.trim() {
                "(" => Elem::Open,
                ")" => Elem::Close,
                "+" => Elem::Plus,
                "*" => Elem::Mul,
                _ => Elem::Value(txt.trim().parse::<u64>().unwrap()),
            })
            .collect::<VecDeque<Elem>>();

        part_one += proceed(line.clone(), false).get().unwrap();
        part_two += proceed(line, true).get().unwrap();
    }
    println!("Part one: {}", part_one);
    println!("Part one: {}", part_two);
}

fn proceed(mut line: VecDeque<Elem>, plus_first: bool) -> Elem {
    while line.contains(&Elem::Open) {
        let mut parenthesis = 0;
        let mut current_op: VecDeque<Elem> = VecDeque::new();
        let mut range = (0, 0);
        let mut new = Elem::Value(0);

        for (i, elem) in line.iter().enumerate() {
            if *elem == Elem::Open {
                parenthesis = 1;
                range.0 = i;
                current_op.clear();
            } else if *elem == Elem::Close {
                parenthesis -= 1;
                // Calculate the parenthesis --
                if parenthesis == 0 {
                    new = match plus_first {
                        true => calculation_plus_first(current_op),
                        false => calculation(current_op),
                    };
                    range.1 = i;
                    break;
                }
            } else {
                current_op.push_back(*elem);
            }
        }

        // Replace and loop --
        for i in (range.0..=range.1).rev() {
            line.remove(i);
        }
        line.insert(range.0, new);
    }

    // Finish the operation --
    match plus_first {
        true => calculation_plus_first(line),
        false => calculation(line),
    }
}

fn calculation(mut input: VecDeque<Elem>) -> Elem {
    let mut result = input.pop_front().unwrap().get().unwrap();

    // display(&input);
    while !input.is_empty() {
        match input.pop_front().unwrap() {
            Elem::Plus => result += input.pop_front().unwrap().get().unwrap(),
            _ => result *= input.pop_front().unwrap().get().unwrap(),
        }
    }

    Elem::Value(result)
}

fn calculation_plus_first(mut input: VecDeque<Elem>) -> Elem {
    while let Some(pos) = input.iter().position(|e| *e == Elem::Plus) {
        let sum = input[pos - 1].get().unwrap() + input[pos + 1].get().unwrap();
        input.remove(pos + 1);
        input.remove(pos);
        input[pos - 1] = Elem::Value(sum);
    }

    calculation(input)
}

#[allow(dead_code)]
fn display(input: &VecDeque<Elem>) {
    for elem in input.iter() {
        match elem {
            Elem::Value(v) => print!(" {} ", v),
            Elem::Plus => print!("+"),
            Elem::Mul => print!("*"),
            Elem::Open => print!(" ( "),
            Elem::Close => print!(" ) "),
        }
    }
    println!();
}
