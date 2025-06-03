#[derive(Debug)]
struct Point {
    row: usize,
    col: usize,
}
impl Point {
    fn new(row: usize, col: usize) -> Point {
        Point { row, col }
    }
}

#[derive(Clone)]
struct Case {
    wall: bool,
    value: i32,
}

#[rustfmt::skip]
impl Point {
    fn neighbours(&self, maze: &[Vec<Case>]) -> Vec<Point> {
        let mut points = Vec::new();
        if self.row > 0                 { points.push(Point::new(self.row - 1, self.col    )); }
        if self.row < maze.len() - 1    { points.push(Point::new(self.row + 1, self.col    )); }
        if self.col > 0                 { points.push(Point::new(self.row    , self.col - 1)); }
        if self.col < maze[0].len() - 1 { points.push(Point::new(self.row    , self.col + 1)); }

        points.retain(|p| !maze[p.row][p.col].wall);
        points
    }
}

fn main() {
    // let (favourite, rows, columns, end_row, end_col) = (10, 6, 10, 4, 7);
    let (favourite, rows, columns, end_row, end_col) = (1362, 200, 200, 39, 31);

    let mut maze: Vec<Vec<Case>> = Vec::new();

    for row in 0..rows {
        maze.push(Vec::new());
        for col in 0..columns {
            if let Some(last) = maze.last_mut() {
                last.push(Case {
                    wall: is_wall(row, col, favourite),
                    value: i32::MAX,
                });
            }
        }
    }

    maze[1][1].value = 0;
    go(&mut maze, Point::new(1, 1));

    println!("Part one: {}", maze[end_row][end_col].value);
    println!(
        "Part two: {}",
        maze.iter()
            .flatten()
            .filter(|&c| !c.wall && c.value <= 50)
            .count()
    );

    // print(&maze);
}

fn go(maze: &mut Vec<Vec<Case>>, current: Point) -> u32 {
    let current_value = maze[current.row][current.col].value;

    for neighbour in current.neighbours(maze) {
        if current_value + 1 < maze[neighbour.row][neighbour.col].value {
            maze[neighbour.row][neighbour.col].value = current_value + 1;
            go(maze, neighbour);
        }
    }

    0
}

fn is_wall(row: usize, col: usize, favourite: usize) -> bool {
    let mut value = col * col + 3 * col + 2 * col * row + row + row * row + favourite;

    let mut tot: u32 = 0;
    while value > 0 {
        if value & 1 == 1 {
            tot += 1;
        }
        value >>= 1
    }

    tot.rem_euclid(2) != 0
}

#[allow(dead_code)]
fn print(maze: &[Vec<Case>]) {
    println!();

    for row in maze.iter() {
        for col in row.iter() {
            if col.wall {
                print!("#");
            } else if col.value == i32::MAX {
                print!(" ");
            } else {
                print!("{}", col.value % 10);
            }
        }
        println!();
    }
}
