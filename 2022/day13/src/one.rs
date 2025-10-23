#[derive(Debug, PartialEq)]
enum Type {
    Open,
    Close,
    Value(u8),
}

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let input = std::fs::read_to_string(path).unwrap();
    let input: Vec<&str> = input.lines().filter(|l| !l.is_empty()).collect();

    let mut total = 0;

    for (i, win) in input.windows(2).step_by(2).enumerate() {
        let mut left = convert(win[0]);
        let mut right = convert(win[1]);

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
                total += i + 1;
                break;
            } else if left[idl] != Type::Close && right[idr] == Type::Close {
                // println!("{i} Invalid because right is shorter than left");
                break;
            } else if a > b {
                // println!("{i} Invalid because left value is higher {a}  {b}");
                break;
            } else if a < b {
                // println!("{i} Valid because left value is lower {a}  {b}");
                total += i + 1;
                break;
            }

            idl += 1;
            idr += 1;
        }
    }
    println!("Part one: {}", total);
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
