#[derive(Debug, Hash, PartialEq, Eq)]
struct Point {
    row: i32,
    col: i32,
}

impl Point {
    fn new(row: i32, col: i32) -> Self {
        Point { row, col }
    }

    fn neighbours(&self, size: i32) -> Vec<Point> {
        let mut n = Vec::new();

        for row in self.row - 1..=self.row + 1 {
            for col in self.col - 1..=self.col + 1 {
                if row >= 0
                    && row < size
                    && col >= 0
                    && col < size
                    && !(row == self.row && col == self.col)
                {
                    n.push(Point::new(row, col));
                }
            }
        }
        n
    }
}

fn main() {
    // let (path, steps) = ("input_example.txt", 4);
    // let (path, steps) = ("input_example.txt", 5);
    let (path, steps) = ("input.txt", 100);

    let input = std::fs::read_to_string(path).unwrap();
    let size = input.lines().count() as i32;

    let mut map = new_map(size, false);
    let mut map_p2 = new_map(size, true);
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.char_indices() {
            if c == '#' {
                map.push(Point::new(row as i32, col as i32));
                map_p2.push(Point::new(row as i32, col as i32));
            }
        }
    }

    for _ in 0..steps {
        map = go(map, size, false);
        map_p2 = go(map_p2, size, true);
    }
    println!("Part one: {}", map.len());
    println!("Part two: {}", map_p2.len());
}

fn go(map: Vec<Point>, size: i32, set_corners: bool) -> Vec<Point> {
    let mut new = new_map(size, set_corners);

    for row in 0..size {
        for col in 0..size {
            if !(set_corners && ((row == 0 || row == size - 1) && (col == 0 || col == size - 1))) {
                let pt = Point::new(row, col);
                let mut neighbours = pt.neighbours(size);
                neighbours.retain(|n| map.contains(n));

                if map.contains(&pt) {
                    if neighbours.len() == 2 || neighbours.len() == 3 {
                        new.push(pt);
                    }
                } else if neighbours.len() == 3 {
                    new.push(pt);
                }
            }
        }
    }

    new
}

fn new_map(size: i32, set_corners: bool) -> Vec<Point> {
    if set_corners {
        vec![
            Point::new(0, 0),
            Point::new(size - 1, 0),
            Point::new(0, size - 1),
            Point::new(size - 1, size - 1),
        ]
    } else {
        Vec::new()
    }
}

fn display(map: &[Point], size: i32) {
    println!("{}", "-".repeat(size as usize * 2));
    for row in 0..size {
        for col in 0..size {
            if map.contains(&Point::new(row, col)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!()
    }
}
