#[derive(Debug)]
struct Disk {
    positions: isize,
    init: isize,
}

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let disks: Vec<Disk> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            let mut it = line.split_whitespace();
            Disk {
                positions: it.nth(3).unwrap().parse::<isize>().unwrap(),
                init: it
                    .last()
                    .unwrap()
                    .chars()
                    .filter(|c| c.is_ascii_digit())
                    .collect::<String>()
                    .parse::<isize>()
                    .unwrap(),
            }
        })
        .collect();

    println!("Part one: {}", brute_force(&disks[..disks.len() - 1]));
    println!("Part two: {}", brute_force(&disks));
}

fn brute_force(disks: &[Disk]) -> isize {
    let mut i: isize = 0;
    loop {
        if disks.iter().enumerate().all(|(index, disk)| {
            (i + disk.init + index as isize + 1).rem_euclid(disk.positions) == 0
        }) {
            return i;
        }

        i += 1;
    }
}
