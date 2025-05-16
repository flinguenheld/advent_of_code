fn main() {
    // let path = "input_example.txt";
    let path = "input.txt";

    let input = std::fs::read_to_string(path).unwrap();

    println!(
        "Part one: {}",
        input.lines().fold(0, |acc, line| {
            let mut real_char_counter = -2;
            let mut escape = false;
            let mut hexa = 0;
            for c in line.chars() {
                real_char_counter += 1;
                if hexa > 0 {
                    hexa -= 1;
                } else if escape {
                    escape = false;
                    real_char_counter -= 1;
                    if c == 'x' {
                        hexa = 2;
                        real_char_counter -= 2;
                    }
                } else if c == '\\' {
                    escape = true;
                }
            }

            acc + line.len() as i32 - real_char_counter
        })
    );

    // --
    println!(
        "Part two: {}",
        input.lines().fold(0, |acc, line| {
            let mut real_char_counter = 4;
            let mut escape = false;
            let mut hexa = 0;
            for c in line.chars() {
                real_char_counter += 1;
                if hexa > 0 {
                    hexa -= 1;
                } else if escape {
                    escape = false;
                    real_char_counter += 2;
                    if c == 'x' {
                        hexa = 2;
                        real_char_counter -= 1;
                    }
                } else if c == '\\' {
                    escape = true;
                }
            }

            acc + real_char_counter - line.len()
        })
    );
}
