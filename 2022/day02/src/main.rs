use std::fs;

#[allow(clippy::identity_op)]
fn main() {
    // let path = "input_example.txt";
    let path = "input.txt";

    let mut total = 0;
    for line in fs::read_to_string(path).unwrap().lines() {
        if let Some((elf, me)) = line.split_once(" ") {
            total += match (elf, me) {
                ("A", "X") => 1 + 3,
                ("A", "Y") => 2 + 6,
                ("A", "Z") => 3 + 0,
                ("B", "X") => 1 + 0,
                ("B", "Y") => 2 + 3,
                ("B", "Z") => 3 + 6,
                ("C", "X") => 1 + 6,
                ("C", "Y") => 2 + 0,
                ("C", "Z") => 3 + 3,
                _ => 0,
            }
        }
    }
    println!("Part one: {}", total);

    let mut total = 0;
    for line in fs::read_to_string(path).unwrap().lines() {
        if let Some((elf, me)) = line.split_once(" ") {
            total += match (elf, me) {
                ("A", "X") => 3 + 0,
                ("A", "Y") => 1 + 3,
                ("A", "Z") => 2 + 6,
                ("B", "X") => 1 + 0,
                ("B", "Y") => 2 + 3,
                ("B", "Z") => 3 + 6,
                ("C", "X") => 2 + 0,
                ("C", "Y") => 3 + 3,
                ("C", "Z") => 1 + 6,
                _ => 0,
            }
        }
    }
    println!("Part two: {}", total);
}
