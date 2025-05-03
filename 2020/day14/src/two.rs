use std::{collections::HashMap, fs};

struct Mask {
    std: u64,
    pre_x: u64,
    x: Vec<u64>,
}
impl Mask {
    fn from(input: &str) -> Self {
        let mut std = 0_u64;
        let mut pre_x = 0_u64;
        let mut x = vec![0_u64];

        for c in input.chars().rev() {
            match c {
                '1' => {
                    std |= 68719476736;
                    std >>= 1;

                    pre_x |= 68719476736;
                    pre_x >>= 1;

                    x.iter_mut().for_each(|v| {
                        *v >>= 1;
                    });
                }
                '0' => {
                    std >>= 1;

                    pre_x |= 68719476736;
                    pre_x >>= 1;

                    x.iter_mut().for_each(|v| {
                        *v >>= 1;
                    });
                }
                'X' => {
                    std >>= 1;
                    pre_x >>= 1;

                    let mut new = x.clone();
                    new.iter_mut().for_each(|v| {
                        *v |= 68719476736;
                        *v >>= 1;
                    });
                    x.iter_mut().for_each(|v| *v >>= 1);
                    x.extend(new);
                }
                _ => {}
            }
        }

        Mask { x, std, pre_x }
    }
}

fn main() {
    // let path = "input_example.txt";
    // let path = "input_example_2.txt";
    let path = "input.txt";

    let mut memory: HashMap<u64, u64> = HashMap::new();

    for block in fs::read_to_string(path).unwrap().split("mask = ") {
        let mut mask: Mask = Mask::from("");
        for (i, line) in block.lines().enumerate() {
            if i == 0 {
                mask = Mask::from(line);
            } else if let Some((mem, value)) = line[4..].split_once("] = ") {
                let mem = mem.parse::<u64>().unwrap();
                let value = value.parse::<u64>().unwrap();

                for m in mask.x.iter() {
                    let mut mem = mem;

                    mem |= mask.std;
                    mem &= mask.pre_x;
                    mem ^= m;

                    memory
                        .entry(mem)
                        .and_modify(|e| *e = value)
                        .or_insert(value);
                }
            }
        }
    }

    println!("Part two: {}", memory.values().sum::<u64>());
}
