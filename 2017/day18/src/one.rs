use std::collections::HashMap;

#[derive(Debug)]
enum RegVal {
    Register(char),
    Value(i64),
}

#[derive(Debug)]
enum Operator {
    Sound(RegVal),
    Set(char, RegVal),
    Add(char, RegVal),
    Mul(char, RegVal),
    Mod(char, RegVal),
    Receive(char),
    Jump(RegVal, RegVal),
}

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let mut registers: HashMap<char, i64> = HashMap::new();
    let instructions: Vec<Operator> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            let fields: Vec<&str> = line.split_whitespace().collect();
            let register = fields[1].chars().next().unwrap();

            if !register.is_ascii_digit() {
                registers.insert(register, 0);
            }

            match fields[0] {
                "snd" => Operator::Sound(wrap_regval(&fields, 1)),
                "set" => Operator::Set(register, wrap_regval(&fields, 2)),
                "add" => Operator::Add(register, wrap_regval(&fields, 2)),
                "mul" => Operator::Mul(register, wrap_regval(&fields, 2)),
                "mod" => Operator::Mod(register, wrap_regval(&fields, 2)),
                "rcv" => Operator::Receive(register),
                _ => Operator::Jump(wrap_regval(&fields, 1), wrap_regval(&fields, 2)),
            }
        })
        .collect();

    let mut index = 0;
    let mut last_played = 0;
    loop {
        if let Some(instruction) = instructions.get(index) {
            match instruction {
                Operator::Sound(val) => {
                    last_played = extract_regval(val, &registers);
                }
                Operator::Set(register, val) => {
                    let val = extract_regval(val, &registers);
                    registers.entry(*register).and_modify(|e| *e = val);
                }
                Operator::Add(register, val) => {
                    let val = extract_regval(val, &registers);
                    registers.entry(*register).and_modify(|e| *e += val);
                }
                Operator::Mul(register, val) => {
                    let val = extract_regval(val, &registers);
                    registers.entry(*register).and_modify(|e| *e *= val);
                }
                Operator::Mod(register, val) => {
                    let val = extract_regval(val, &registers);
                    registers.entry(*register).and_modify(|e| *e %= val);
                }
                Operator::Receive(register) => {
                    if let Some(r) = registers.get(register) {
                        if *r != 0 {
                            println!("Part one: {}", last_played);
                            break;
                        }
                    }
                }
                Operator::Jump(val, times) => {
                    let val = extract_regval(val, &registers);
                    let times = extract_regval(times, &registers);
                    if val > 0 {
                        index = (index as i64 + times) as usize;
                        continue;
                    }
                }
            }
        } else {
            println!("what ?");
            break;
        }

        index += 1;
    }
}

fn extract_regval(from: &RegVal, registers: &HashMap<char, i64>) -> i64 {
    match from {
        RegVal::Register(r) => *registers.get(r).unwrap(),
        RegVal::Value(v) => *v,
    }
}
fn wrap_regval(fields: &[&str], index: usize) -> RegVal {
    let blah = fields.get(index).unwrap();
    if let Ok(value) = blah.parse::<i64>() {
        RegVal::Value(value)
    } else {
        RegVal::Register(blah.chars().next().unwrap())
    }
}
