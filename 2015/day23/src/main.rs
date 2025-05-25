#[derive(Debug)]
enum Instruction {
    Half(usize),
    Triple(usize),
    Increment(usize),
    Jump(i32),
    JumpEven(usize, i32),
    JumpOne(usize, i32),
}

fn main() {
    let input = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|l| {
            let mut it = l.split_whitespace();
            let first = it.next().unwrap();
            let second = it.next().unwrap();
            let second = match &second[..1] {
                "a" => 0,
                "b" => 1,
                _ => second.parse::<i32>().unwrap(),
            };

            match first {
                "hlf" => Instruction::Half(second as usize),
                "tpl" => Instruction::Triple(second as usize),
                "inc" => Instruction::Increment(second as usize),
                "jmp" => Instruction::Jump(second),
                "jie" => Instruction::JumpEven(
                    second as usize,
                    it.next().unwrap().parse::<i32>().unwrap(),
                ),
                _ => Instruction::JumpOne(
                    second as usize,
                    it.next().unwrap().parse::<i32>().unwrap(),
                ),
            }
        })
        .collect::<Vec<Instruction>>();

    println!("Part one: {}", process(&input, 0));
    println!("Part two: {}", process(&input, 1));
}

fn process(input: &[Instruction], a: u32) -> u32 {
    let mut registers = [a, 0];
    let mut index = 0_i32;

    while let Some(instruction) = input.get(index as usize) {
        let mut jump = 0;

        match instruction {
            Instruction::Half(i) => registers[*i] /= 2,
            Instruction::Triple(i) => registers[*i] *= 3,
            Instruction::Increment(i) => registers[*i] += 1,
            Instruction::Jump(v) => jump += *v,
            Instruction::JumpEven(i, v) => {
                if registers[*i] % 2 == 0 {
                    jump += *v
                }
            }
            Instruction::JumpOne(i, v) => {
                if registers[*i] == 1 {
                    jump += *v
                }
            }
        }

        if jump == 0 {
            index += 1;
        } else if index + jump < 0 {
            break;
        } else {
            index += jump;
        }
    }
    registers[1]
}
