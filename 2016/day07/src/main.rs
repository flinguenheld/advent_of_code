fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";
    // let path = "input_example_2.txt";

    println!(
        "Part one: {}",
        std::fs::read_to_string(path)
            .unwrap()
            .lines()
            .fold(0, |acc, line| {
                let mut current: Vec<char> = Vec::new();
                let mut outside_ok = false;
                let mut inside_clear = true;

                for c in line.chars() {
                    match c {
                        '[' => {
                            outside_ok = outside_ok || contains_abba(&current);
                            current.clear();
                        }
                        ']' => {
                            inside_clear = inside_clear && !contains_abba(&current);
                            current.clear();
                        }
                        _ => {
                            current.push(c);
                        }
                    }
                }
                outside_ok = outside_ok || contains_abba(&current);

                match outside_ok && inside_clear {
                    true => acc + 1,
                    false => acc,
                }
            })
    );

    // --

    println!(
        "Part two: {}",
        std::fs::read_to_string(path)
            .unwrap()
            .lines()
            .fold(0, |acc, line| {
                let mut current: Vec<char> = Vec::new();
                let mut babs: Vec<Vec<char>> = Vec::new();
                let mut abas = Vec::new();

                for c in line.chars() {
                    match c {
                        '[' => {
                            babs.append(&mut list_abas(&current, false));
                            current.clear();
                        }
                        ']' => {
                            abas.append(&mut list_abas(&current, true));
                            current.clear();
                        }
                        _ => {
                            current.push(c);
                        }
                    }
                }
                babs.append(&mut list_abas(&current, false));

                match abas.iter().any(|aba| babs.contains(aba)) {
                    true => acc + 1,
                    false => acc,
                }
            })
    );
}

fn contains_abba(text: &[char]) -> bool {
    for win in text.windows(4) {
        if win[0] == win[3] && win[1] == win[2] && win[0] != win[1] {
            return true;
        }
    }
    false
}

// List aba or babs and directly convert them to the opposite
fn list_abas(text: &[char], bab: bool) -> Vec<Vec<char>> {
    let mut found = Vec::new();
    for win in text.windows(3) {
        if win[0] == win[2] && win[0] != win[1] {
            found.push(match bab {
                true => vec![win[1], win[0], win[1]],
                false => vec![win[0], win[1], win[0]],
            });
        }
    }
    found
}
