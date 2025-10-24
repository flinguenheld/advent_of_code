use std::collections::HashSet;

fn main() {
    let (path, which_row) = ("input.txt", 2000000);
    // let (path, which_row) = ("input_example.txt", 10);

    let input = std::fs::read_to_string(path);

    let mut forbidden_places: HashSet<(i32, i32)> = HashSet::new();
    for line in input.unwrap().lines() {
        let fields: Vec<&str> = line.split_whitespace().collect();

        let col_sensor = to_int(fields[2]);
        let row_sensor = to_int(fields[3]);

        let col_beacon = to_int(fields[8]);
        let row_beacon = to_int(fields[9]);

        // Get the manhattan distance
        let dist = (row_sensor - row_beacon).abs() + (col_sensor - col_beacon).abs();

        let dist_to_row = (which_row - row_sensor).abs();
        if dist_to_row < dist {
            for col in (col_sensor - (dist - dist_to_row))..=(col_sensor + (dist - dist_to_row)) {
                forbidden_places.insert((which_row, col));
            }
        }
    }

    println!(
        "Part one: {}",
        forbidden_places
            .iter()
            .filter(|(row, _)| *row == which_row)
            .count()
            - 1
    );
}

fn to_int(txt: &str) -> i32 {
    txt.chars()
        .filter(|c| c.is_ascii_digit() || *c == '-')
        .collect::<String>()
        .parse::<i32>()
        .unwrap()
}
