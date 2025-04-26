use std::{cmp::Ordering, fs};

fn main() {
    // let path = "input_example.txt";
    let path = "input.txt";

    let input = fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| l.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut product = 0;
    find(Vec::new(), &input, &mut product, 2);
    println!("Part one: {}", product);

    product = 0;
    find(Vec::new(), &input, &mut product, 3);
    println!("Part two: {}", product);
}

fn find(saved: Vec<u32>, input: &[u32], product: &mut u32, length: usize) {
    if *product == 0 {
        match saved.iter().sum::<u32>().cmp(&2020) {
            Ordering::Equal => {
                if saved.len() == length {
                    *product = saved.iter().product::<u32>();
                }
            }
            Ordering::Less => {
                if saved.len() < length {
                    for value in input.iter() {
                        let mut s = saved.clone();
                        s.push(*value);

                        find(s, input, product, length);
                    }
                }
            }
            Ordering::Greater => {}
        }
    }
}
