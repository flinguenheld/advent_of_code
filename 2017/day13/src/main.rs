fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let mut scanners: Vec<usize> = Vec::new();
    let mut index = 0;
    for line in std::fs::read_to_string(path).unwrap().lines() {
        let (left, right) = line.split_once(": ").unwrap();

        let left = left.parse::<usize>().unwrap();
        let right = right.parse::<usize>().unwrap();

        while left > index {
            scanners.push(0);
            index += 1;
        }

        scanners.push(right);
        index += 1;
    }
    // dbg!(&scanners);

    let mut total = 0;
    for (i, scanner) in scanners.iter().enumerate().skip(1) {
        if *scanner > 0 && (i).rem_euclid(2 * (scanner - 1)) == 0 {
            total += i * scanner;
        }
    }

    println!("Part one: {}", total);

    // --
    let mut delay = 1;
    while scanners
        .iter()
        .enumerate()
        .filter(|(_, s)| **s > 0)
        .any(|(i, s)| (i + delay).rem_euclid(2 * (s - 1)) == 0)
    {
        delay += 1;
    }
    println!("Part two: {}", delay);
}
