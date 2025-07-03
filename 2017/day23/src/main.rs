use std::collections::HashMap;

#[derive(Debug)]
enum RegVal {
    Register(char),
    Value(i32),
}

#[derive(Debug)]
enum Instruction {
    Set(char, RegVal),
    Sub(char, RegVal),
    Mul(char, RegVal),
    Jnz(RegVal, RegVal),
}

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let mut registers: HashMap<char, i32> = ('a'..='h').map(|r| (r, 0)).collect();

    let input = std::fs::read_to_string(path).unwrap();
    let instructions: Vec<Instruction> = input
        .lines()
        .map(|line| {
            let fields: Vec<&str> = line.split_whitespace().collect();

            let field_1 = into_regval(fields[1]);
            let field_2 = into_regval(fields[2]);

            match fields[0] {
                "set" => Instruction::Set(fields[1].chars().next().unwrap(), field_2),
                "sub" => Instruction::Sub(fields[1].chars().next().unwrap(), field_2),
                "mul" => Instruction::Mul(fields[1].chars().next().unwrap(), field_2),
                _ => Instruction::Jnz(field_1, field_2),
            }
        })
        .collect();

    // dbg!(&instructions);

    // registers.entry('a').and_modify(|e| *e = 1);
    let mut counter = 0;
    let mut index: i32 = 0;
    while let Some(inst) = instructions.get(index as usize) {
        match inst {
            Instruction::Set(reg, regval) => {
                let b = into_int(regval, &registers);
                registers.entry(*reg).and_modify(|e| *e = b);
            }
            Instruction::Sub(reg, regval) => {
                let b = into_int(regval, &registers);
                registers.entry(*reg).and_modify(|e| *e -= b);
            }
            Instruction::Mul(reg, regval) => {
                counter += 1;
                let b = into_int(regval, &registers);
                registers.entry(*reg).and_modify(|e| *e *= b);
            }
            Instruction::Jnz(a, b) => {
                if into_int(a, &registers) != 0 {
                    index += into_int(b, &registers);
                    if index < 0 {
                        break;
                    }
                    continue;
                }
            }
        }

        index += 1;
    }

    println!("Part one: {}", counter);
    // println!("Part two: {}", registers.get(&'h').unwrap());
}

fn into_regval(txt: &str) -> RegVal {
    if let Ok(value) = txt.parse::<i32>() {
        RegVal::Value(value)
    } else {
        RegVal::Register(txt.chars().next().unwrap())
    }
}
fn into_int(regval: &RegVal, registers: &HashMap<char, i32>) -> i32 {
    match regval {
        RegVal::Register(r) => *registers.get(r).unwrap(),
        RegVal::Value(v) => *v,
    }
}
