#[derive(Debug, PartialEq, Eq)]
struct Point {
    row: i32,
    col: i32,
}

fn main() {
    let (path, check_high) = ("input.txt", 10);
    // let (path, check_high) = ("input_example.txt", 8);

    let mut points: Vec<Point> = Vec::new();
    let mut velocities: Vec<Point> = Vec::new();

    for line in std::fs::read_to_string(path).unwrap().lines() {
        let mut it = line.split(|c: char| [',', '>'].contains(&c));
        let col = to_int(it.next().unwrap());
        let row = to_int(it.next().unwrap());

        let velo_col = to_int(it.next().unwrap());
        let velo_row = to_int(it.next().unwrap());

        points.push(Point { row, col });
        velocities.push(Point {
            row: velo_row,
            col: velo_col,
        });
    }

    let mut time = 0;
    loop {
        time += 1;

        for (pt, velo) in points.iter_mut().zip(velocities.iter()) {
            pt.row += velo.row;
            pt.col += velo.col;
        }

        let min_row = points.iter().min_by_key(|pt| pt.row).unwrap().row;
        let max_row = points.iter().max_by_key(|pt| pt.row).unwrap().row;

        if max_row - min_row < check_high {
            println!("Part one:");
            print(&points);
            println!("Part two: {}", time);
            break;
        }
    }
}

fn print(points: &[Point]) {
    let min_row = points.iter().min_by_key(|pt| pt.row).unwrap().row;
    let max_row = points.iter().max_by_key(|pt| pt.row).unwrap().row;

    let min_col = points.iter().min_by_key(|pt| pt.col).unwrap().col;
    let max_col = points.iter().max_by_key(|pt| pt.col).unwrap().col;

    for row in min_row..=max_row {
        for col in min_col..=max_col {
            let pt = Point { row, col };
            if points.iter().any(|p| *p == pt) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn to_int(txt: &str) -> i32 {
    txt.chars()
        .filter(|c| c.is_ascii_digit() || *c == '-')
        .collect::<String>()
        .parse::<i32>()
        .unwrap()
}
