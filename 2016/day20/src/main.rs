#[derive(Debug)]
struct Addr {
    min: u64,
    max: u64,
}

fn main() {
    let (path, to) = ("input.txt", 4294967295_u64);
    // let (path, to) = ("input_example.txt", 9_u64);

    let mut addresses: Vec<Addr> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| {
            let (left, right) = l.split_once('-').unwrap();

            Addr {
                min: left.parse::<u64>().unwrap(),
                max: right.parse::<u64>().unwrap(),
            }
        })
        .collect();

    addresses.sort_by_key(|a| a.min);

    // --
    let mut previous = addresses.first().unwrap();
    for new in addresses.iter().skip(1) {
        if new.min <= previous.max + 1 && new.max > previous.max {
            previous = new;
        }
    }

    println!("Part one: {}", previous.max + 1);

    // --
    let mut total = 0;
    let mut current_max = addresses.first().unwrap().max;
    for new in addresses.iter().skip(1) {
        if new.min > current_max + 1 {
            total += new.min - current_max - 1;
        }

        if new.max > current_max {
            current_max = new.max;
        }
    }

    if current_max < to {
        total += to - current_max;
    }

    println!("Part two: {}", total);
}
