use std::{collections::VecDeque, mem::swap};

fn main() {
    // let input = "1";
    let input = "3113322113";

    let input: VecDeque<char> = input.chars().collect();

    println!("Part one: {}", lenght_after(40, input.clone()));
    println!("Part two: {}", lenght_after(50, input));
}

fn lenght_after(n: u32, mut input: VecDeque<char>) -> usize {
    for _ in 0..n {
        let mut line: VecDeque<char> = VecDeque::new();
        while let Some(value) = input.pop_front() {
            let mut nb = 1_u8;
            while let Some(next) = input.pop_front() {
                match next == value {
                    true => nb += 1,
                    false => {
                        input.push_front(next);
                        break;
                    }
                }
            }
            line.push_back((48 + nb) as char);
            line.push_back(value);
        }
        swap(&mut input, &mut line);
    }

    input.len()
}
