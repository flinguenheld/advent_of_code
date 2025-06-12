#[derive(Debug)]
struct Point {
    row: usize,
    col: usize,
}

impl Point {
    fn new(row: usize, col: usize) -> Self {
        Point { row, col }
    }

    fn neighbours(&self, tree: &[Vec<Node>]) -> Vec<Point> {
        let mut n = Vec::new();

        if self.row > 0 {
            n.push(Point::new(self.row - 1, self.col));
        }
        if self.row < tree.len() - 1 {
            n.push(Point::new(self.row + 1, self.col));
        }
        if self.col > 0 {
            n.push(Point::new(self.row, self.col - 1));
        }
        if self.col < tree[0].len() - 1 {
            n.push(Point::new(self.row, self.col + 1));
        }

        n
    }
}

#[derive(Debug, Clone, Copy)]
struct Node {
    size: u32,
    used: u32,

    move_value: u32,
}

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let mut nodes: Vec<Vec<Node>> = Vec::new();

    for line in std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .filter(|l| l.starts_with("/dev/"))
    {
        let fields: Vec<&str> = line.split_whitespace().collect();

        let (_, row) = &fields[0][16..].split_once('-').unwrap();
        let row = row
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<usize>()
            .unwrap();

        let size = fields[1]
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<u32>()
            .unwrap();
        let used = fields[2]
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<u32>()
            .unwrap();

        if nodes.is_empty() || row > nodes.len() - 1 {
            nodes.push(Vec::new());
        }

        if let Some(current_row) = nodes.get_mut(row) {
            current_row.push(Node {
                size,
                used,
                move_value: u32::MAX,
            });
        }
    }

    let mut total = 0;
    for (row, r) in nodes.iter().enumerate() {
        for (col, node) in r.iter().enumerate() {
            if node.used > 0 {
                total += nodes.iter().enumerate().fold(0, |acc, (i, ro)| {
                    acc + ro.iter().enumerate().fold(0, |acc2, (j, n)| {
                        if node.used <= n.size - n.used && !(i == row && j == col) {
                            acc2 + 1
                        } else {
                            acc2
                        }
                    })
                })
            }
        }
    }
    println!("Part one: {}", total);

    // --
    // Move the empty node beside the goal
    let mut empty_node = Point::new(0, 0);

    for (row, r) in nodes.iter().enumerate() {
        for (col, node) in r.iter().enumerate() {
            if node.used == 0 {
                empty_node.row = row;
                empty_node.col = col;
            }
        }
    }

    set_values(&mut nodes, empty_node, 0);

    // Get the shortest path to be on the left of the goal
    let goal = Point::new(0, nodes[0].len() - 1);
    let mut moves = nodes[goal.row][goal.col - 1].move_value;

    // Then comming back to the left costs 5:
    //                1           2           3           4           5
    // . _ G .     . G _ .     . G . .     . G . .     . G . .     _ G . .
    // . . . .     . . . .     . . _ .     . _ . .     _ . . .     . . . .
    //
    // So 5 * the amount of nodes to the top left
    moves += 5 * (goal.col as u32 - 1);
    // Last move to put the goal on 0x0
    moves += 1;

    println!("Part two: {}", moves);
}

fn set_values(map: &mut Vec<Vec<Node>>, current_position: Point, current_value: u32) {
    map[current_position.row][current_position.col].move_value = current_value;

    for neighbour in current_position.neighbours(map) {
        let node = map[neighbour.row][neighbour.col];
        if node.move_value == u32::MAX || node.move_value > current_value + 1 {
            set_values(map, neighbour, current_value + 1);
        }
    }
}

#[allow(dead_code)]
fn print(tree: &[Vec<Node>]) {
    for row in tree.iter() {
        for node in row.iter() {
            print!("{:5}", node.move_value);
            // if node.used == 0 {
            //     print!("____/{:<4}", node.size);
            // } else {
            //     print!("{:4}/{:<4}", node.used, node.size);
            // }
        }
        println!();
    }
}
