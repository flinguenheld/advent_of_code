fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";
    // let path = "input_example_2.txt";

    let input = std::fs::read_to_string(path).unwrap();

    println!(
        "Part one: {}",
        input
            .lines()
            .filter(|line| line.split_whitespace().enumerate().all(|(i, val)| {
                for field in line.split_whitespace().skip(i + 1) {
                    if val == field {
                        return false;
                    }
                }
                true
            }))
            .count()
    );

    println!(
        "Part two: {}",
        input
            .lines()
            .filter(|line| {
                line.split_whitespace().enumerate().all(|(i, val)| {
                    for field in line.split_whitespace().skip(i + 1) {
                        if field.chars().count() == val.chars().count() {
                            let mut a = field.chars().collect::<Vec<char>>();
                            let mut b = val.chars().collect::<Vec<char>>();
                            a.sort_unstable();
                            b.sort_unstable();
                            if a == b {
                                return false;
                            }
                        }
                    }
                    true
                })
            })
            .count()
    )
}
