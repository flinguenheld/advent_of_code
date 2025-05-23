fn main() {
    let input = 34_000_000;

    // That's crazy but such a large array is not a problem :|
    // And with a vec, one or twenty millions are ok :|
    // let mut houses = [0; 1_000_000];
    let mut houses = vec![0; 5_000_000];

    for elf in 1..input {
        houses.iter_mut().step_by(elf).skip(1).for_each(|house| {
            *house += elf * 10;
        });
    }
    println!(
        "Part one: {}",
        houses.iter().position(|v| *v >= input).unwrap()
    );

    // houses = [0; 1_000_000];
    let mut houses = vec![0; 5_000_000];
    for elf in 1..input {
        for (counter, house) in houses.iter_mut().step_by(elf).skip(1).enumerate() {
            if counter >= 50 {
                break;
            }
            *house += elf * 11;
        }
    }

    println!(
        "Part two: {}",
        houses.iter().position(|v| *v >= input).unwrap()
    );
}

// Work in around seven minutes
fn part_two(nb: u32) -> u32 {
    let mut total = 0;
    for i in 1..=nb {
        if nb % i == 0 && nb / i < 50 {
            total += i;
        }
    }
    total * 11
}

fn part_one(nb: u32) -> u32 {
    let mut total = 0;
    for i in 1..=nb {
        if nb % i == 0 {
            total += i;
        }
    }
    total * 10
}
