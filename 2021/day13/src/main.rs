use std::collections::HashSet;

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let input = std::fs::read_to_string(path).unwrap();

    let mut dots: Vec<(u16, u16)> = input
        .lines()
        .filter(|l| l.starts_with(|c: char| c.is_ascii_digit()))
        .map(|l| {
            let (col, row) = l.split_once(',').unwrap();
            (row.parse().unwrap(), col.parse().unwrap())
        })
        .collect();

    for (step, fold) in input.lines().filter(|l| l.starts_with("fol")).enumerate() {
        let val: u16 = fold
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse()
            .unwrap();

        if fold.contains('y') {
            dots.iter_mut()
                .filter(|(row, _)| *row > val)
                .for_each(|(row, _)| *row = val - (*row - val));
        } else {
            dots.iter_mut()
                .filter(|(_, col)| *col > val)
                .for_each(|(_, col)| *col = val - (*col - val));
        }

        if step == 0 {
            let hash: HashSet<&(u16, u16)> = HashSet::from_iter(dots.iter());
            println!("Part one: {}", hash.len());
        }
    }

    println!("Part two:");
    print(&dots);
}

fn print(dots: &[(u16, u16)]) {
    for row in 0..=dots.iter().max_by_key(|v| v.0).unwrap().0 {
        for col in 0..=dots.iter().max_by_key(|v| v.1).unwrap().1 {
            if dots.contains(&(row, col)) {
                print!("#");
            } else {
                print!(".")
            }
        }
        println!();
    }
}
