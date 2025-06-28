use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
enum RegVal {
    Register(char),
    Value(i64),
}

#[derive(Debug)]
enum Operator {
    Send(RegVal),
    Set(char, RegVal),
    Add(char, RegVal),
    Mul(char, RegVal),
    Mod(char, RegVal),
    Receive(char),
    Jump(RegVal, RegVal),
}

#[derive(Debug)]
struct Programm {
    registers: HashMap<char, i64>,
    queue: VecDeque<i64>,
    current_op: usize,
}

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let mut program_0 = Programm {
        registers: HashMap::new(),
        queue: VecDeque::new(),
        current_op: 0,
    };
    let mut program_1 = Programm {
        registers: HashMap::new(),
        queue: VecDeque::new(),
        current_op: 0,
    };

    let instructions: Vec<Operator> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            let fields: Vec<&str> = line.split_whitespace().collect();
            let register = fields[1].chars().next().unwrap();

            if !register.is_ascii_digit() {
                program_0.registers.insert(register, 0);
                program_1.registers.insert(register, 0);
            }

            match fields[0] {
                "snd" => Operator::Send(wrap_regval(&fields, 1)),
                "set" => Operator::Set(register, wrap_regval(&fields, 2)),
                "add" => Operator::Add(register, wrap_regval(&fields, 2)),
                "mul" => Operator::Mul(register, wrap_regval(&fields, 2)),
                "mod" => Operator::Mod(register, wrap_regval(&fields, 2)),
                "rcv" => Operator::Receive(register),
                _ => Operator::Jump(wrap_regval(&fields, 1), wrap_regval(&fields, 2)),
            }
        })
        .collect();

    // Init p
    program_0.registers.insert('p', 0);
    program_1.registers.insert('p', 1);

    // GO
    let mut counter = 0;
    loop {
        go(&instructions, &mut program_0, &mut program_1);
        counter += go(&instructions, &mut program_1, &mut program_0);

        if program_0.queue.is_empty() && program_1.queue.is_empty() {
            break;
        }
    }
    println!("Part two: {}", counter);
}

fn go(instructions: &[Operator], from: &mut Programm, to: &mut Programm) -> u32 {
    let mut counter = 0;

    loop {
        if let Some(instruction) = instructions.get(from.current_op) {
            match instruction {
                Operator::Send(val) => {
                    let val = extract_regval(val, &from.registers);
                    to.queue.push_back(val);
                    counter += 1;
                }
                Operator::Set(register, val) => {
                    let val = extract_regval(val, &from.registers);
                    from.registers.entry(*register).and_modify(|e| *e = val);
                }
                Operator::Add(register, val) => {
                    let val = extract_regval(val, &from.registers);
                    from.registers.entry(*register).and_modify(|e| *e += val);
                }
                Operator::Mul(register, val) => {
                    let val = extract_regval(val, &from.registers);
                    from.registers.entry(*register).and_modify(|e| *e *= val);
                }
                Operator::Mod(register, val) => {
                    let val = extract_regval(val, &from.registers);
                    from.registers.entry(*register).and_modify(|e| *e %= val);
                }
                Operator::Receive(register) => {
                    if let Some(val) = from.queue.pop_front() {
                        from.registers.entry(*register).and_modify(|e| *e = val);
                    } else {
                        break;
                    }
                }
                Operator::Jump(val, times) => {
                    let val = extract_regval(val, &from.registers);
                    let times = extract_regval(times, &from.registers);
                    if val > 0 {
                        from.current_op = (from.current_op as i64 + times) as usize;
                        continue;
                    }
                }
            }
        } else {
            println!("what ?");
            break;
        }

        from.current_op += 1;
    }

    counter
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
