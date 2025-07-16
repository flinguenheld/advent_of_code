use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
enum Op {
    Addr,
    Addi,

    Mulr,
    Muli,

    Banr,
    Bani,

    Borr,
    Bori,

    Setr,
    Seti,

    Gtir,
    Gtri,
    Gtrr,

    Eqir,
    Eqri,
    Eqrr,
}

impl Op {
    fn apply(&self, a: usize, b: usize, c: usize, mut registers: [i32; 4]) -> [i32; 4] {
        match self {
            Self::Addr => registers[c] = registers[a] + registers[b],
            Self::Addi => registers[c] = registers[a] + b as i32,

            Self::Mulr => registers[c] = registers[a] * registers[b],
            Self::Muli => registers[c] = registers[a] * b as i32,

            Self::Banr => registers[c] = registers[a] & registers[b],
            Self::Bani => registers[c] = registers[a] & b as i32,

            Self::Borr => registers[c] = registers[a] | registers[b],
            Self::Bori => registers[c] = registers[a] | b as i32,

            Self::Setr => registers[c] = registers[a],
            Self::Seti => registers[c] = a as i32,

            Self::Gtir => {
                registers[c] = match a as i32 > registers[b] {
                    true => 1,
                    false => 0,
                }
            }
            Self::Gtri => {
                registers[c] = match registers[a] > b as i32 {
                    true => 1,
                    false => 0,
                }
            }
            Self::Gtrr => {
                registers[c] = match registers[a] > registers[b] {
                    true => 1,
                    false => 0,
                }
            }

            Self::Eqir => {
                registers[c] = match a as i32 == registers[b] {
                    true => 1,
                    false => 0,
                }
            }
            Self::Eqri => {
                registers[c] = match registers[a] == b as i32 {
                    true => 1,
                    false => 0,
                }
            }
            Self::Eqrr => {
                registers[c] = match registers[a] == registers[b] {
                    true => 1,
                    false => 0,
                }
            }
        }

        registers
    }
}

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let input = std::fs::read_to_string(path).unwrap();
    let input: Vec<&str> = input.lines().collect();

    let ops = [
        Op::Addr,
        Op::Addi,
        Op::Mulr,
        Op::Muli,
        Op::Setr,
        Op::Seti,
        Op::Banr,
        Op::Bani,
        Op::Bori,
        Op::Borr,
        Op::Gtir,
        Op::Gtri,
        Op::Gtrr,
        Op::Eqir,
        Op::Eqri,
        Op::Eqrr,
    ];

    let mut count = 0;
    let mut indexes_to_sort: Vec<HashSet<usize>> = vec![(0..16).collect(); 16];

    for win in input.windows(3).step_by(4) {
        if win[0].starts_with("Before") {
            let before = register_from(win[0]);
            let val = values_from(win[1]);
            let after = register_from(win[2]);

            let mut valids = 0;
            for (i, op) in ops.iter().enumerate() {
                if op.apply(val[1] as usize, val[2] as usize, val[3] as usize, before) != after {
                    indexes_to_sort[val[0]].remove(&i);
                } else {
                    valids += 1;
                }
            }

            if valids >= 3 {
                count += 1;
            }
        }
    }

    println!("Part one: {}", count);

    // Who is who ?
    let mut alones: HashSet<usize> = HashSet::new();
    while alones.len() < ops.len() {
        for alone in indexes_to_sort.iter().filter(|a| a.len() == 1) {
            alones.insert(*alone.iter().next().unwrap());
        }

        indexes_to_sort
            .iter_mut()
            .filter(|a| a.len() > 1)
            .for_each(|a| a.retain(|b| !alones.contains(b)));
    }

    let sorted: Vec<Op> = indexes_to_sort.iter().flatten().map(|v| ops[*v]).collect();

    // Run the program
    let pos_program = input.iter().rposition(|l| l.starts_with("After")).unwrap();

    let mut registers = [0, 0, 0, 0];
    for line in input.iter().skip(pos_program + 4) {
        let values = values_from(line);
        registers = sorted[values[0]].apply(values[1], values[2], values[3], registers);
    }

    println!("Part two: {}", registers[0]);
}

fn values_from(txt: &str) -> [usize; 4] {
    let mut fields = txt.split_whitespace();
    [
        fields.next().unwrap().parse::<usize>().unwrap(),
        fields.next().unwrap().parse::<usize>().unwrap(),
        fields.next().unwrap().parse::<usize>().unwrap(),
        fields.next().unwrap().parse::<usize>().unwrap(),
    ]
}
fn register_from(txt: &str) -> [i32; 4] {
    let mut fields = txt.split(',');
    [
        to_int(fields.next().unwrap()),
        to_int(fields.next().unwrap()),
        to_int(fields.next().unwrap()),
        to_int(fields.next().unwrap()),
    ]
}

fn to_int(txt: &str) -> i32 {
    txt.chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<i32>()
        .unwrap()
}
