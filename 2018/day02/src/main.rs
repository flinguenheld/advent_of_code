use std::collections::HashSet;

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";
    // let path = "input_example_2.txt";

    let mut two = 0;
    let mut three = 0;
    for line in std::fs::read_to_string(path).unwrap().lines() {
        let letters: HashSet<char> = line.chars().collect();
        let counters: Vec<usize> = letters
            .iter()
            .map(|l| line.chars().filter(|c| c == l).count())
            .collect();

        if counters.contains(&2) {
            two += 1;
        }
        if counters.contains(&3) {
            three += 1;
        }
    }

    println!("Part one: {}", two * three);

    // --
    let input = std::fs::read_to_string(path).unwrap();
    let lines: Vec<&str> = input.lines().collect();

    'done: for line in lines.iter() {
        'next_line: for to_check in lines.iter().filter(|l| *l != line) {
            let mut index = usize::MAX;
            for (i, (a, b)) in line.chars().zip(to_check.chars()).enumerate() {
                if a != b {
                    if index < usize::MAX {
                        continue 'next_line;
                    }
                    index = i;
                }
            }

            println!(
                "Part two: {}",
                line.char_indices()
                    .filter(|(i, _)| *i != index)
                    .map(|(_, c)| c)
                    .collect::<String>()
            );
            break 'done;
        }
    }
}
