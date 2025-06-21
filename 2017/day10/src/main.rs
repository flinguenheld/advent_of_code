use std::fmt::Write;

fn main() {
    let (path, size) = ("input.txt", 255_u8);
    // let (path, size) = ("input_example.txt", 4);

    let lengths: Vec<u8> = std::fs::read_to_string(path)
        .unwrap()
        .split(',')
        .map(|v| {
            v.chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse::<u8>()
                .unwrap()
        })
        .collect();

    let mut list: Vec<u8> = (0..=size).collect();

    let mut current_index = 0;
    let mut skip = 0;
    run(&lengths, &mut list, &mut current_index, &mut skip);

    println!("Part one: {}", list[0] as u32 * list[1] as u32);

    // --
    let mut list: Vec<u8> = (0..=size).collect();
    let mut lengths = std::fs::read_to_string(path)
        .unwrap()
        .chars()
        .filter(|c| *c != '\n')
        .map(|v| v as u8)
        .collect::<Vec<u8>>();
    lengths.extend([17, 31, 73, 47, 23].iter());

    let mut current_index = 0;
    let mut skip = 0;
    for _ in 0..64 {
        run(&lengths, &mut list, &mut current_index, &mut skip);
    }

    println!(
        "Part two: {}",
        list.windows(16)
            .step_by(16)
            .fold(String::new(), |mut hash, win| {
                write!(hash, "{:02x}", win.iter().fold(0, |acc, v| acc ^ v)).ok();
                hash
            })
    );
}

fn run(lengths: &[u8], list: &mut [u8], current_index: &mut usize, skip_size: &mut usize) {
    let list_length = list.len();
    for length in lengths.iter() {
        let temp = list.to_owned();

        let mut rev = *current_index as i32 + *length as i32 - 1;
        for _ in 0..*length {
            list[current_index.rem_euclid(list_length)] =
                temp[rev.rem_euclid(list_length as i32) as usize];

            *current_index += 1;
            rev -= 1;
        }

        *current_index += *skip_size;
        *skip_size += 1;
    }
}
