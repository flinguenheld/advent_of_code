use std::cmp;

type Coord = ((i64, i64), (i64, i64));

fn main() {
    let (path, max_row) = ("input.txt", 4_000_000);
    // let (path, max_row) = ("input_example.txt", 20);

    let input = std::fs::read_to_string(path);

    let mut coordinates: Vec<((i64, i64), (i64, i64))> = Vec::new();

    for line in input.unwrap().lines() {
        let fields: Vec<&str> = line.split_whitespace().collect();

        let col_sensor = to_int(fields[2]);
        let row_sensor = to_int(fields[3]);

        let col_beacon = to_int(fields[8]);
        let row_beacon = to_int(fields[9]);

        coordinates.push(((row_sensor, col_sensor), (row_beacon, col_beacon)));
    }

    for row in 0..max_row {
        if let Some(col_found) = is_there_a_column_gap(&coordinates, row) {
            println!("Part two: {}", col_found * 4_000_000 + row);
            break;
        }
    }
}
// 841265410 -> too low
// 841265411 -> too low

// Get all ranges for this row and check if there is any gap
fn is_there_a_column_gap(coordinates: &[Coord], which_row: i64) -> Option<i64> {
    let mut ranges: Vec<(i64, i64)> = Vec::new();

    for coordinate in coordinates.iter() {
        let row_sensor = coordinate.0 .0;
        let col_sensor = coordinate.0 .1;

        let row_beacon = coordinate.1 .0;
        let col_beacon = coordinate.1 .1;

        // Get the manhattan distance
        let dist = (row_sensor - row_beacon).abs() + (col_sensor - col_beacon).abs();

        let dist_to_row = (which_row - row_sensor).abs();
        if dist_to_row < dist {
            // The forbidden shape is a diamon, so for this row:
            let left = col_sensor - (dist - dist_to_row);
            let right = col_sensor + (dist - dist_to_row);
            ranges.push((left, right));
        }
    }
    ranges.sort_by_key(|r| r.1);
    ranges.sort_by_key(|r| r.0);

    let mut current = ranges.first().unwrap().1;
    for range in ranges.iter() {
        if range.0 <= (current + 1) {
            current = cmp::max(current, range.1);
        } else {
            return Some(range.0 - 1);
        }
    }
    None
}

fn to_int(txt: &str) -> i64 {
    txt.chars()
        .filter(|c| c.is_ascii_digit() || *c == '-')
        .collect::<String>()
        .parse::<i64>()
        .unwrap()
}
