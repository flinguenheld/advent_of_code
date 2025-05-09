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

    let mut total = 0;
    for line in fs::read_to_string(path).unwrap().lines() {
        // println!("Line: '{}'", line);

        let line = line.replace(")", " )");
        let mut line = line
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

        // display(&line);

        while line.contains(&Elem::Open) {
            // display(&line);
            let mut parenthesis = 0;
            let mut current: VecDeque<Elem> = VecDeque::new();
            let mut from = 0;
            let mut to = 0;
            let mut new = None;

            for (i, elem) in line.iter().enumerate() {
                if *elem == Elem::Open {
                    parenthesis = 1;
                    from = i;
                    current.clear();
                } else if *elem == Elem::Close {
                    parenthesis -= 1;
                    if parenthesis == 0 {
                        // Replace current by the result
                        new = Some(calculation(current));
                        to = i;
                        break;
                    }
                } else {
                    current.push_back(*elem);
                }
            }

            for i in (from..=to).rev() {
                line.remove(i);
            }
            // display(&line);
            line.insert(from, new.unwrap());
            // display(&line);
            // }
        }

        // display(&line);
        let last = calculation(line);

        // println!("Value of line: {}", last.get().unwrap());
        total += last.get().unwrap();

        // println!("{:?}", line);
    }
    println!("Part one: {}", total);
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
