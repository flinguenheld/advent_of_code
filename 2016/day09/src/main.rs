fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let line: String = std::fs::read_to_string(path)
        .unwrap()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || "()".contains(*c))
        .collect();

    println!("Part one: {}", count(&line, false));
    println!("Part two: {}", count(&line, true));
}

fn count(line: &str, improved_format: bool) -> usize {
    let mut counter = 0;
    let mut paren_start = 0;
    let mut ignore = false;

    let mut skip = 0;

    for (index, c) in line.char_indices() {
        if index >= skip {
            if c == '(' {
                ignore = true;
                paren_start = index;
            } else if c == ')' {
                ignore = false;

                if let Some((amount, times)) = &line[paren_start + 1..index].split_once('x') {
                    let amount = amount.parse::<usize>().unwrap();
                    let times = times.parse::<usize>().unwrap();

                    counter += match improved_format {
                        false => amount * times,
                        true => count(&line[index + 1..index + 1 + amount], true) * times,
                    };
                    skip = index + amount + 1;
                    continue;
                }
            }

            if !ignore {
                counter += 1;
            }
        }
    }

    counter
}
