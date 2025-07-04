use std::cmp::Ordering;

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let pins: Vec<(u32, u32)> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            let (left, right) = line.split_once('/').unwrap();
            let left = left.parse::<u32>().unwrap();
            let right = right.parse::<u32>().unwrap();

            (left, right)
        })
        .collect();

    let mut strongest = 0;
    let mut longest = (0, 0);

    build(&pins, Vec::new(), &mut strongest, &mut longest);

    println!("Part one: {}", strongest);
    println!("Part two: {}", longest.1);
}

fn build(
    pins: &[(u32, u32)],
    current: Vec<(u32, u32)>,
    strongest: &mut u32,
    longest: &mut (usize, u32),
) {
    let s = current.iter().fold(0, |acc, (a, b)| acc + a + b);
    *strongest = s.max(*strongest);

    match current.len().cmp(&longest.0) {
        Ordering::Greater => {
            longest.0 = current.len();
            longest.1 = s;
        }
        Ordering::Equal => {
            longest.1 = s.max(longest.1);
        }
        _ => {}
    }

    if let Some((_, last_b)) = current.last() {
        for (a, b) in pins.iter().filter(|(a, b)| {
            !current.contains(&(*a, *b))
                && !current.contains(&(*b, *a))
                && *a > 0
                && (last_b == a || last_b == b)
        }) {
            let mut c = current.clone();
            if last_b == a {
                c.push((*a, *b));
            } else {
                c.push((*b, *a));
            }

            build(pins, c, strongest, longest);
        }
    } else {
        for next_pin in pins.iter().filter(|(a, _)| *a == 0) {
            let mut c = current.clone();
            c.push(*next_pin);

            build(pins, c, strongest, longest);
        }
    }
}
