use std::{collections::HashMap, fs};

struct Mask {
    x_0: u64,
    x_1: u64,
}
impl Mask {
    fn from(input: &str) -> Self {
        let mut x_0 = 0_u64;
        let mut x_1 = 0_u64;

        for c in input.chars().rev() {
            match c {
                '1' => {
                    x_0 |= 68719476736;
                    x_0 >>= 1;
                    x_1 |= 68719476736;
                    x_1 >>= 1;
                }
                '0' => {
                    x_0 >>= 1;
                    x_1 >>= 1;
                }
                'X' => {
                    x_0 >>= 1;
                    x_1 |= 68719476736;
                    x_1 >>= 1;
                }
                _ => {}
            }
        }

        Mask { x_0, x_1 }
    }
}

fn main() {
    // let path = "input_example.txt";
    let path = "input.txt";

    let mut memory: HashMap<u32, u64> = HashMap::new();

    for block in fs::read_to_string(path).unwrap().split("mask = ") {
        let mut mask: Mask = Mask::from("");
        for (i, line) in block.lines().enumerate() {
            if i == 0 {
                mask = Mask::from(line);
            } else if let Some((mem, value)) = line[4..].split_once("] = ") {
                let mem = mem.parse::<u32>().unwrap();
                let mut value = value.parse::<u64>().unwrap();

                value |= mask.x_0;
                value &= mask.x_1;

                memory
                    .entry(mem)
                    .and_modify(|e| *e = value)
                    .or_insert(value);
            }
        }
    }

    println!("Part one: {}", memory.values().sum::<u64>());
}
