fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let mut counter = 0;
    let mut counter_garbage = 0;

    let mut open = 0;
    let mut garbage = false;

    let mut previous = 'a';

    for c in std::fs::read_to_string(path)
        .unwrap()
        .chars()
        .filter(|c| *c != '\n')
    {
        if previous == '!' && c == '!' {
            previous = 'a';
            continue;
        } else if previous != '!' {
            if garbage {
                if c == '>' {
                    garbage = false;
                } else if c != '!' {
                    counter_garbage += 1;
                }
            } else if c == '<' {
                garbage = true;
            } else if c == '{' {
                open += 1;
            } else if c == '}' {
                counter += open;
                open -= 1;
            }
        }

        previous = c;
    }

    println!("Part one: {}", counter);
    println!("Part two: {}", counter_garbage);
}
