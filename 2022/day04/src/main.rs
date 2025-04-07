use std::fs;

fn main() {
    // let path = "input_example.txt";
    let path = "input.txt";

    let (part_one, part_two) =
        fs::read_to_string(path)
            .unwrap()
            .lines()
            .fold((0, 0), |(p1, p2), l| {
                let vals = l
                    .split(|c| ['-', ','].contains(&c))
                    .map(|nb| nb.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();

                if !(vals[1] < vals[2] || vals[0] > vals[3]) {
                    if (vals[0] >= vals[2] && vals[1] <= vals[3])
                        || (vals[2] >= vals[0] && vals[3] <= vals[1])
                    {
                        (p1 + 1, p2 + 1)
                    } else {
                        (p1, p2 + 1)
                    }
                } else {
                    (p1, p2)
                }
            });

    println!("part one: {}", part_one);
    println!("part two: {}", part_two);
}
