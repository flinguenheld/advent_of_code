enum Step {
    Spin(usize),
    Swap(usize, usize),
    SwapValues(char, char),
}

fn main() {
    let (path, from, to) = ("input.txt", 'a', 'p');
    // let (path, from, to) = ("input_example.txt", 'a', 'e');

    let programs_orginal: Vec<char> = (from..=to).collect();

    let steps: Vec<Step> = std::fs::read_to_string(path)
        .unwrap()
        .trim()
        .split(',')
        .map(|dance| {
            let kind = dance.chars().next().unwrap();

            if kind == 's' {
                Step::Spin(dance[1..].parse::<usize>().unwrap())
            } else {
                let (left, right) = dance[1..].split_once('/').unwrap();

                if kind == 'x' {
                    Step::Swap(
                        left.parse::<usize>().unwrap(),
                        right.parse::<usize>().unwrap(),
                    )
                } else {
                    Step::SwapValues(left.chars().next().unwrap(), right.chars().next().unwrap())
                }
            }
        })
        .collect();

    let mut programs = programs_orginal.clone();
    programs = dance(programs, &steps);
    println!("Part one: {}", programs.iter().collect::<String>());

    // --
    // Find the dance length to come back to the original
    let mut i = 1;
    while programs != programs_orginal {
        programs = dance(programs, &steps);
        i += 1;
    }

    // Move close to the end
    let mut j = (1_000_000_000 / i) * i;
    while j < 1_000_000_000 {
        programs = dance(programs, &steps);
        j += 1;
    }

    println!("Part two: {}", programs.iter().collect::<String>());
}

fn dance(mut programs: Vec<char>, steps: &[Step]) -> Vec<char> {
    for step in steps.iter() {
        match step {
            Step::Spin(times) => programs.rotate_right(*times),
            Step::Swap(a, b) => programs.swap(*a, *b),
            Step::SwapValues(a, b) => {
                let a = programs.iter().position(|v| v == a).unwrap();
                let b = programs.iter().position(|v| v == b).unwrap();

                programs.swap(a, b)
            }
        }
    }
    programs
}
