#[derive(Debug, PartialEq, Clone, Copy)]
enum Type {
    Open,
    Close,
    Value(u8),
}

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let two = convert("[[2]]");
    let six = convert("[[6]]");

    let input = std::fs::read_to_string(path).unwrap();
    let mut input: Vec<Vec<Type>> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(convert)
        .collect();

    input.push(two.clone());
    input.push(six.clone());
    let mut index = 0;

    while index < (input.len() - 1) {
        if !is_left_lower(&input, index) {
            input.swap(index, index + 1);
            index = 0;
            continue;
        }

        index += 1;
    }

    println!(
        "Part two: {}",
        input.iter().enumerate().fold(1, |acc, (i, line)| {
            if *line == two || *line == six {
                acc * (i + 1)
            } else {
                acc
            }
        })
    );
}

fn is_left_lower(input: &[Vec<Type>], index: usize) -> bool {
    let mut left = input[index].clone();
    let mut right = input[index + 1].clone();

    let mut idl = 0;
    let mut idr = 0;

    loop {
        if left[idl] == Type::Open && right[idr] != Type::Open && right[idr] != Type::Close {
            right.insert(idr + 1, Type::Close);
            right.insert(idr, Type::Open);
            continue;
        }
        if right[idr] == Type::Open && left[idl] != Type::Open && left[idl] != Type::Close {
            left.insert(idl + 1, Type::Close);
            left.insert(idl, Type::Open);
            continue;
        }

        let a = match left[idl] {
            Type::Value(v) => v,
            _ => 255,
        };
        let b = match right[idr] {
            Type::Value(v) => v,
            _ => 255,
        };

        if left[idl] == Type::Close && right[idr] != Type::Close {
            // println!("{i} Valid because left is shorter than right");
            return true;
        } else if left[idl] != Type::Close && right[idr] == Type::Close {
            // println!("{i} Invalid because right is shorter than left");
            return false;
        } else if a > b {
            // println!("{i} Invalid because left value is higher {a}  {b}");
            return false;
        } else if a < b {
            // println!("{i} Valid because left value is lower {a}  {b}");
            return true;
        }

        idl += 1;
        idr += 1;
    }
}

fn convert(txt: &str) -> Vec<Type> {
    let mut to_convert = String::new();
    let mut values: Vec<Type> = Vec::new();

    for c in txt.chars() {
        match c {
            '[' => values.push(Type::Open),
            ']' => {
                if !to_convert.is_empty() {
                    values.push(Type::Value(to_convert.parse::<u8>().unwrap()));
                    to_convert.clear();
                }
                values.push(Type::Close);
            }
            ',' => {
                if !to_convert.is_empty() {
                    values.push(Type::Value(to_convert.parse::<u8>().unwrap()));
                    to_convert.clear();
                }
            }
            _ => to_convert.push(c),
        }
    }
    values
}
