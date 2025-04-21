use colorized::colorize_print;
use std::fs;

#[derive(Clone)]
struct Case {
    elevation: char,
    value: u32,
}

struct Point {
    r: usize,
    c: usize,
}

#[rustfmt::skip]
impl Point {
    fn up(&self) -> Option<Self> {
        match self.r > 0 { true => Some(Point { r: self.r - 1, c: self.c, }),
                           false => None, }
    }
    fn down(&self, input: &[Vec<Case>]) -> Option<Self> {
        match self.r < input.len() - 1 { true => Some(Point { r: self.r + 1, c: self.c, }),
                                         false => None, }
    }
    fn left(&self) -> Option<Self> {
        match self.c > 0 { true => Some(Point { r: self.r, c: self.c - 1, }),
                           false => None, }
    }
    fn right(&self, input: &[Vec<Case>]) -> Option<Self> {
        match self.c < input[0].len() - 1 { true => Some(Point { r: self.r, c: self.c + 1, }),
                                            false => None, }
    }
}

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let input: Vec<Vec<Case>> = fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| {
            l.chars()
                .map(|elevation| Case {
                    elevation,
                    value: 0,
                })
                .collect()
        })
        .collect();

    let start = position(&input, 'S');
    let end = position(&input, 'E');

    let mut input_from_s = input.clone();
    number_them(&start, &mut input_from_s, 'S', 0);

    let mut mini = input_from_s[end.r][end.c].value;

    display(&input_from_s);
    println!("Part one: {}", mini);

    for (r, row) in input.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            if col.elevation == 'a' {
                let mut input_temp = input.clone();
                number_them(&Point { r, c }, &mut input_temp, 'S', 0);
                if input_temp[end.r][end.c].value > 0 && input_temp[end.r][end.c].value < mini {
                    mini = input_temp[end.r][end.c].value;
                    display(&input_temp);
                }
            }
        }
    }
    println!("Part two: {}", mini);
}

fn position(input: &[Vec<Case>], elevation: char) -> Point {
    for (r, row) in input.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            if col.elevation == elevation {
                return Point { r, c };
            }
        }
    }
    Point { r: 0, c: 0 }
}

fn number_them(cur: &Point, input: &mut [Vec<Case>], previous_elevation: char, value: u32) {
    // Only accept the End from a z
    if input[cur.r][cur.c].elevation == 'E' && previous_elevation != 'z' {
        return;
    }

    // Only accept smaller or higher + 1
    if input[cur.r][cur.c].elevation as u8 > previous_elevation as u8 + 1
        && previous_elevation != 'S'
    {
        return;
    }

    if (input[cur.r][cur.c].elevation as u8 <= previous_elevation as u8 + 1
        || previous_elevation == 'S' && input[cur.r][cur.c].elevation == 'a')
        && (input[cur.r][cur.c].value == 0 || value < input[cur.r][cur.c].value)
    {
        if input[cur.r][cur.c].elevation != 'S' {
            input[cur.r][cur.c].value = value;
        }

        if let Some(next) = cur.up() {
            number_them(&next, input, input[cur.r][cur.c].elevation, value + 1)
        }
        if let Some(next) = cur.down(input) {
            number_them(&next, input, input[cur.r][cur.c].elevation, value + 1)
        }
        if let Some(next) = cur.left() {
            number_them(&next, input, input[cur.r][cur.c].elevation, value + 1)
        }
        if let Some(next) = cur.right(input) {
            number_them(&next, input, input[cur.r][cur.c].elevation, value + 1)
        }
    }
}

fn display(input: &[Vec<Case>]) {
    for row in input.iter() {
        for col in row.iter() {
            match col.elevation {
                'a' => colorize_print(format!("{:3}", col.value), colorized::Colors::RedFg),
                'b' => colorize_print(format!("{:3}", col.value), colorized::Colors::GreenFg),
                'c' => colorize_print(format!("{:3}", col.value), colorized::Colors::YellowFg),
                'd' => colorize_print(format!("{:3}", col.value), colorized::Colors::MagentaFg),
                'e' => colorize_print(format!("{:3}", col.value), colorized::Colors::BlueFg),
                'f' => colorize_print(format!("{:3}", col.value), colorized::Colors::BrightBlackFg),
                'g' => colorize_print(format!("{:3}", col.value), colorized::Colors::RedFg),
                'h' => colorize_print(format!("{:3}", col.value), colorized::Colors::GreenFg),
                'i' => colorize_print(format!("{:3}", col.value), colorized::Colors::CyanFg),
                'j' => colorize_print(format!("{:3}", col.value), colorized::Colors::WhiteFg),
                'k' => colorize_print(format!("{:3}", col.value), colorized::Colors::YellowFg),
                'l' => colorize_print(format!("{:3}", col.value), colorized::Colors::BlueFg),
                'm' => colorize_print(format!("{:3}", col.value), colorized::Colors::BrightCyanFg),
                'n' => colorize_print(format!("{:3}", col.value), colorized::Colors::BlueFg),
                'o' => colorize_print(format!("{:3}", col.value), colorized::Colors::GreenFg),
                'p' => colorize_print(format!("{:3}", col.value), colorized::Colors::BrightRedFg),
                'q' => colorize_print(format!("{:3}", col.value), colorized::Colors::BlackFg),
                'r' => colorize_print(format!("{:3}", col.value), colorized::Colors::MagentaFg),
                's' => colorize_print(format!("{:3}", col.value), colorized::Colors::BrightGreenFg),
                't' => colorize_print(format!("{:3}", col.value), colorized::Colors::RedFg),
                'u' => colorize_print(format!("{:3}", col.value), colorized::Colors::MagentaFg),
                'v' => colorize_print(format!("{:3}", col.value), colorized::Colors::BrightWhiteFg),
                'w' => colorize_print(format!("{:3}", col.value), colorized::Colors::WhiteFg),
                'x' => colorize_print(format!("{:3}", col.value), colorized::Colors::BlueFg),
                'y' => colorize_print(format!("{:3}", col.value), colorized::Colors::GreenFg),
                'z' => colorize_print(format!("{:3}", col.value), colorized::Colors::RedFg),
                _ => print!("{:3}", col.value),
            }
        }
        println!();
    }
    println!("{}", "-".repeat(input[0].len() * 3));
}
