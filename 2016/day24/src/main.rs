use itertools::Itertools;
use std::collections::HashMap;

type Hvac = Vec<Vec<char>>;
type Cache = Vec<Vec<u32>>;

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
struct Point {
    row: usize,
    col: usize,
}
impl Point {
    fn new(row: usize, col: usize) -> Self {
        Point { row, col }
    }

    // No checks thanks to the borders
    fn neighbours(&self, hvac: &[Vec<char>]) -> Vec<Point> {
        let mut neigh = Vec::new();

        if hvac[self.row - 1][self.col] != '#' {
            neigh.push(Point::new(self.row - 1, self.col));
        }
        if hvac[self.row + 1][self.col] != '#' {
            neigh.push(Point::new(self.row + 1, self.col));
        }
        if hvac[self.row][self.col - 1] != '#' {
            neigh.push(Point::new(self.row, self.col - 1));
        }
        if hvac[self.row][self.col + 1] != '#' {
            neigh.push(Point::new(self.row, self.col + 1));
        }

        neigh
    }
}

fn main() {
    // let path = "input.txt";
    let path = "input_example.txt";

    let hvac: Hvac = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    // Get the highest value
    let highest = *hvac
        .iter()
        .flatten()
        .filter(|case| case.is_ascii_digit())
        .max_by_key(|case| **case as u8)
        .unwrap() as u32
        - 48;

    // Get all target positions
    let targets: Vec<Point> = (0..=highest)
        .map(|target| get_point(&hvac, target).unwrap())
        .collect();

    // Get the shortest distance for each couple of targets
    // (Save both sides (0,1) & (1,0) ..)
    let mut distances: HashMap<(u32, u32), u32> = HashMap::new();
    let mut cache = vec![vec![u32::MAX; hvac[0].len()]; hvac.len()];

    for from in targets.iter() {
        cache = fill_values(&hvac, cache, *from, 0);

        let from_value = hvac[from.row][from.col] as u32 - 48;
        for (i, to) in targets.iter().enumerate().filter(|(_, pt)| *pt != from) {
            distances.insert((from_value, i as u32), cache[to.row][to.col]);
        }

        cache.iter_mut().for_each(|row| row.fill(u32::MAX));
    }

    let mut mini = u32::MAX;
    for mut permutation in (1..=highest).permutations(highest as usize) {
        permutation.insert(0, 0);

        mini = std::cmp::min(
            mini,
            permutation.windows(2).fold(0, |acc, win| {
                acc + distances.get(&(win[0], win[1])).unwrap()
            }),
        )
    }
    println!("Part one: {}", mini);

    // --
    let mut mini = u32::MAX;
    for mut permutation in (1..=highest).permutations(highest as usize) {
        permutation.insert(0, 0);
        permutation.push(0);

        mini = std::cmp::min(
            mini,
            permutation.windows(2).fold(0, |acc, win| {
                acc + distances.get(&(win[0], win[1])).unwrap()
            }),
        )
    }
    println!("Part two: {}", mini);
}

fn fill_values(hvac: &Hvac, mut cache: Cache, current_point: Point, current_value: u32) -> Cache {
    if current_value < cache[current_point.row][current_point.col] {
        cache[current_point.row][current_point.col] = current_value;

        for neighbour in current_point.neighbours(hvac) {
            cache = fill_values(hvac, cache, neighbour, current_value + 1);
        }
    }
    // print_cache(&cache);
    cache
}

fn get_point(hvac: &Hvac, value: u32) -> Option<Point> {
    let value = (value as u8 + 48) as char;

    for (r, row) in hvac.iter().enumerate() {
        for (c, node) in row.iter().enumerate() {
            if *node == value {
                return Some(Point::new(r, c));
            }
        }
    }
    None
}

#[allow(dead_code)]
fn print(hvac: &Hvac) {
    println!();
    for row in hvac.iter() {
        for node in row.iter() {
            print!("{}", node);
        }
        println!();
    }
}
#[allow(dead_code)]
fn print_cache(hvac: &Cache) {
    println!();
    for row in hvac.iter() {
        for node in row.iter() {
            if *node == u32::MAX {
                print!(" .");
            } else {
                print!("{:2}", node)
            }
        }
        println!();
    }
}
