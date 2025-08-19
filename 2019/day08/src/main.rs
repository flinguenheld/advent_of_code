fn main() {
    // let (max_row, max_col, path) = (2, 3, "input_example.txt");
    // let (max_row, max_col, path) = (2, 2, "input_example.txt");
    let (max_row, max_col, path) = (6, 25, "input.txt");

    let input: Vec<char> = std::fs::read_to_string(path)
        .unwrap()
        .trim()
        .chars()
        .collect();

    let input: Vec<&[char]> = input
        .windows(max_col * max_row)
        .step_by(max_col * max_row)
        .collect();

    let mini = input
        .iter()
        .filter(|layer| layer.contains(&'0'))
        .min_by_key(|layer| layer.iter().filter(|c| **c == '0').count())
        .unwrap();

    println!(
        "Part one: {}",
        mini.iter().filter(|c| **c == '1').count() * mini.iter().filter(|c| **c == '2').count()
    );

    // --
    let mut image = Vec::new();

    'blah: for index in 0..input[0].len() {
        for layer in input.iter() {
            if layer[index] == '0' {
                image.push(' ');
                continue 'blah;
            } else if layer[index] == '1' {
                image.push('#');
                continue 'blah;
            }
        }

        image.push(' ');
    }

    println!("Part two: ");
    print(&image, max_col);
}

fn print(input: &[char], width: usize) {
    for win in input.windows(width).step_by(width) {
        for c in win.iter() {
            print!("{}", c);
        }
        println!();
    }
}
