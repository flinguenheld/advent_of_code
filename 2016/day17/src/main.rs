use chksum_md5 as md5;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Point {
    row: usize,
    col: usize,
}
impl Point {
    fn new(row: usize, col: usize) -> Point {
        Point { row, col }
    }
}

#[rustfmt::skip]
impl Point {
    fn up   (&self, map: &[Vec<char>]) -> bool { map[self.row - 1][self.col] != '#' }
    fn down (&self, map: &[Vec<char>]) -> bool { map[self.row + 1][self.col] != '#' }
    fn left (&self, map: &[Vec<char>]) -> bool { map[self.row][self.col - 1] != '#' }
    fn right(&self, map: &[Vec<char>]) -> bool { map[self.row][self.col + 1] != '#' }
}

fn main() {
    // let input = "ihgpwlah";
    // let input = "kglvqrro";
    // let input = "ulqzkmiv";
    let input = "hhhxzeay";

    let map: Vec<Vec<char>> = std::fs::read_to_string("map.txt")
        .unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    let shortest = go(&map, Point { row: 1, col: 1 }, input, String::new(), true);
    let longest = go(&map, Point { row: 1, col: 1 }, input, String::new(), false);

    println!("Part one: {}", shortest);
    println!("Part two: {}", longest.chars().count());
}

fn go(map: &Vec<Vec<char>>, position: Point, input: &str, path: String, shortest: bool) -> String {
    let mut mini = String::new();

    if map[position.row][position.col] == 'V' {
        return path;
    }
    let hash = md5::chksum(format!("{}{}", input, path)).unwrap();
    let hash: Vec<u8> = hash.as_bytes()[..2]
        .iter()
        .flat_map(|v| to_hex(*v))
        .collect();

    if hash[0] > 10 && position.up(map) {
        let next = Point::new(position.row - 2, position.col);
        let path = format!("{}{}", path, 'U');
        mini = get_min_max(mini, go(map, next, input, path, shortest), shortest);
    }
    if hash[1] > 10 && position.down(map) {
        let next = Point::new(position.row + 2, position.col);
        let path = format!("{}{}", path, 'D');
        mini = get_min_max(mini, go(map, next, input, path, shortest), shortest);
    }
    if hash[2] > 10 && position.left(map) {
        let next = Point::new(position.row, position.col - 2);
        let path = format!("{}{}", path, 'L');
        mini = get_min_max(mini, go(map, next, input, path, shortest), shortest);
    }
    if hash[3] > 10 && position.right(map) {
        let next = Point::new(position.row, position.col + 2);
        let path = format!("{}{}", path, 'R');
        mini = get_min_max(mini, go(map, next, input, path, shortest), shortest);
    }

    mini
}

fn get_min_max(l: String, r: String, shortest: bool) -> String {
    match (l.is_empty(), r.is_empty(), shortest) {
        (false, false, true) => std::cmp::min_by_key(l, r, |txt| txt.chars().count()),
        (false, false, false) => std::cmp::max_by_key(l, r, |txt| txt.chars().count()),
        (true, false, _) => r,
        _ => l,
    }
}

fn to_hex(mut value: u8) -> Vec<u8> {
    let mut hex = Vec::new();
    while value > 0 {
        hex.insert(0, value % 16);
        value /= 16;
    }
    while hex.len() < 2 {
        hex.insert(0, 0);
    }
    hex
}

#[allow(dead_code)]
fn print(map: &[Vec<char>]) {
    for row in map.iter() {
        for col in row.iter() {
            print!("{}", col);
        }
        println!();
    }
}
