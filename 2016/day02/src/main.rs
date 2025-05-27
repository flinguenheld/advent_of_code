#[derive(Clone, Copy)]
struct Point {
    row: u8,
    col: u8,
}

impl Point {
    fn go(self, to: char, keypad: &[Vec<char>]) -> Point {
        let mut pt = self;
        match to {
            'U' => pt.row = pt.row.saturating_sub(1),
            'D' => pt.row += 1,
            'L' => pt.col = pt.col.saturating_sub(1),
            _ => pt.col += 1,
        }

        match pt.value(keypad).is_some() {
            true => pt,
            false => self,
        }
    }

    fn value(&self, keypad: &[Vec<char>]) -> Option<char> {
        if let Some(row) = keypad.get(self.row as usize) {
            if let Some(c) = row.get(self.col as usize) {
                if *c != 'X' {
                    return Some(*c);
                }
            }
        }
        None
    }
}

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let keypad_1 = vec![
        vec!['1', '2', '3'],
        vec!['4', '5', '6'],
        vec!['7', '8', '9'],
    ];
    let keypad_2 = vec![
        vec!['X', 'X', '1', 'X', 'X'],
        vec!['X', '2', '3', '4', 'X'],
        vec!['5', '6', '7', '8', '9'],
        vec!['X', 'A', 'B', 'C', 'X'],
        vec!['X', 'X', 'D', 'X', 'X'],
    ];

    println!(
        "Part one: {}",
        proceed(path, Point { row: 1, col: 1 }, &keypad_1)
    );
    println!(
        "Part two: {}",
        proceed(path, Point { row: 2, col: 0 }, &keypad_2)
    );
}

fn proceed(path: &str, mut pt: Point, keypad: &[Vec<char>]) -> String {
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            line.chars().for_each(|c| pt = pt.go(c, keypad));
            pt.value(keypad).unwrap()
        })
        .collect()
}
