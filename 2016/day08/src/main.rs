//           path, rows,  cols
const DATA: (&str, usize, usize) = ("input.txt", 6, 50);
// const DATA: (&str, usize, usize) = ("input_example.txt", 3, 7);

struct Screen([[char; DATA.2]; DATA.1]);

impl Screen {
    fn new() -> Self {
        Screen([['.'; DATA.2]; DATA.1])
    }
    fn rotate_row(mut self, index: usize, times: usize) -> Screen {
        // Get all '#' positions in the row
        let active: Vec<usize> = self.0[index]
            .iter()
            .enumerate()
            .filter(|(_, &c)| c == '#')
            .map(|(i, _)| (i + times).rem_euclid(DATA.2))
            .collect();

        self.0[index]
            .iter_mut()
            .enumerate()
            .for_each(|(i, c)| match active.contains(&i) {
                true => *c = '#',
                false => *c = '.',
            });

        self
    }
    fn rotate_col(mut self, index: usize, times: usize) -> Screen {
        // Get all '#' positions in the col
        let active: Vec<usize> = self
            .0
            .iter()
            .enumerate()
            .filter(|(_, &c)| c[index] == '#')
            .map(|(i, _)| (i + times).rem_euclid(DATA.1))
            .collect();

        self.0
            .iter_mut()
            .enumerate()
            .for_each(|(i, c)| match active.contains(&i) {
                true => c[index] = '#',
                false => c[index] = '.',
            });

        self
    }
    fn set_rectangle(mut self, col: usize, row: usize) -> Screen {
        for r in 0..row {
            for c in 0..col {
                self.0[r][c] = '#';
            }
        }

        self
    }
    fn print(&self) {
        for row in self.0.iter() {
            for col in row.iter() {
                print!("{}", col);
            }
            println!();
        }
        println!();
    }
}

fn main() {
    let mut screen = Screen::new();

    for line in std::fs::read_to_string(DATA.0).unwrap().lines() {
        if line.starts_with("rect") {
            if let Some((col, row)) = &line[5..].split_once('x') {
                screen = screen
                    .set_rectangle(col.parse::<usize>().unwrap(), row.parse::<usize>().unwrap());
            }
        } else if line.contains("row") {
            if let Some((y, times)) = &line[13..].split_once(" by ") {
                screen =
                    screen.rotate_row(y.parse::<usize>().unwrap(), times.parse::<usize>().unwrap());
            }
        } else if let Some((y, times)) = &line[16..].split_once(" by ") {
            screen =
                screen.rotate_col(y.parse::<usize>().unwrap(), times.parse::<usize>().unwrap());
        }
    }

    println!(
        "Part one: {}",
        screen.0.iter().flatten().filter(|&&c| c == '#').count()
    );
    println!("Part two:");
    screen.print();
}
