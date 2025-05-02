use std::fs;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
struct Bus {
    number: i64,
    start_from_0: i64,
}

fn main() {
    // let path = "input_example.txt";
    let path = "input.txt";

    let input = fs::read_to_string(path).unwrap();
    let (depart, buses) = input.split_once('\n').unwrap();

    let depart = depart.parse::<i64>().unwrap();
    let buses = buses
        .trim()
        .split(',')
        .enumerate()
        .filter(|(_, v)| v.chars().all(|c| c.is_ascii_digit()))
        .map(|(i, v)| Bus {
            number: v.parse::<i64>().unwrap(),
            start_from_0: i as i64,
        })
        .collect::<Vec<Bus>>();

    let mut next_depart = depart;
    loop {
        next_depart += 1;

        if let Some(bus) = buses.iter().filter(|t| next_depart % t.number == 0).last() {
            println!("bus: {}", bus.number);
            println!("Part one: {}", bus.number * (next_depart - depart));
            break;
        }
    }

    // Part two --
    // Chinese remainder theorem
    let m = buses.iter().map(|b| b.number).product::<i64>();
    let mi = buses.iter().map(|b| m / b.number).collect::<Vec<i64>>();
    let reverse_mods = mi
        .iter()
        .zip(buses.iter())
        .map(|(mi, b)| modular_inverse(*mi, b.number))
        .collect::<Vec<i64>>();

    let fin = buses
        .iter()
        .zip(mi.iter().zip(reverse_mods.iter()))
        .map(|(b, (mi, rev))| -b.start_from_0 * mi * rev)
        .sum::<i64>()
        .rem_euclid(m);

    println!("Part two calculation: {}", fin);
    // println!("Part two with loop: {}", find_time(&buses, false));
}

fn modular_inverse(a: i64, b: i64) -> i64 {
    let mut y = 0;
    loop {
        y += 1;
        if y * a % b == 1 {
            break;
        }
    }
    y
}

// Impossible -_-
fn find_time(buses: &[Bus], print: bool) -> i64 {
    let max = buses.iter().min().unwrap();
    let mut depart = max.number;
    loop {
        depart += max.number + max.start_from_0;

        if print {
            print!("{} --> ", depart);
            for bus in buses.iter() {
                if (depart - max.start_from_0 + bus.start_from_0) % bus.number == 0 {
                    print!("   D");
                } else {
                    print!("   .");
                }
            }
            println!();
        }

        if buses
            .iter()
            .all(|bus| (depart - max.start_from_0 + bus.start_from_0) % bus.number == 0)
        {
            return depart;
        }
    }
}
