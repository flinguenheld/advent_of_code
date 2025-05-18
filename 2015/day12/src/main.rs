fn main() {
    // let path = "input_example.txt";
    let path = "input.txt";

    let input = std::fs::read_to_string(path).unwrap();

    println!("Part one: {}", nb(&input));

    // --
    let mut total = 0;
    let mut brackets: Vec<String> = Vec::new();
    let mut regular = String::new();

    for c in input.chars() {
        if c == '{' || c == '[' {
            brackets.push(String::new());
        } else if c == '}' || c == ']' {
            if let Some(last) = brackets.pop() {
                if (c == '}' && !last.contains("red")) || c == ']' {
                    if let Some(penultimate) = brackets.last_mut() {
                        *penultimate = format!("{},{},", penultimate, nb(&last));
                    } else {
                        total += nb(&last);
                    }
                }
            }
        }

        if let Some(last) = brackets.last_mut() {
            last.push(c);
        } else {
            regular.push(c);
        }
    }
    total += nb(&regular);

    println!("Part two: {}", total);
}

fn nb(input: &str) -> i32 {
    input.split(',').fold(0, |acc, elem| {
        acc + elem
            .chars()
            .filter(|c| c.is_ascii_digit() || *c == '-')
            .collect::<String>()
            .parse::<i32>()
            .unwrap_or(0)
    })
}
