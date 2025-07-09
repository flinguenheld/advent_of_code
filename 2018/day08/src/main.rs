fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let input: Vec<u32> = std::fs::read_to_string(path)
        .unwrap()
        .split_whitespace()
        .map(|field| field.parse::<u32>().unwrap())
        .collect();

    // dbg!(&input);
    println!("Part one: {}", sum_data(&input).1);
    println!("Part two: {}", get_data(&input).1);
}

fn get_data(input: &[u32]) -> (usize, u32) {
    let mut position = 2;
    let mut data = 0;
    let mut cache: Vec<u32> = Vec::new();

    for _ in 0..input[0] {
        let (p, d) = get_data(&input[position..]);
        position += p;
        cache.push(d);
    }

    let data_len = input[1] as usize;
    if input[0] == 0 {
        data += input[position..position + data_len].iter().sum::<u32>();
    } else {
        for i in input[position..position + data_len].iter() {
            if let Some(v) = cache.get(*i as usize - 1) {
                data += v;
            }
        }
    }

    position += data_len;

    (position, data)
}

fn sum_data(input: &[u32]) -> (usize, u32) {
    let mut position = 2;
    let mut data = 0;

    for _ in 0..input[0] {
        let (p, d) = sum_data(&input[position..]);
        position += p;
        data += d;
    }

    let data_len = input[1] as usize;
    data += input[position..position + data_len].iter().sum::<u32>();
    position += data_len;

    (position, data)
}
