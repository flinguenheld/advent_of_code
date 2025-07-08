use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    row: u32,
    col: u32,
}

fn main() {
    let (path, max_d) = ("input.txt", 10_000);
    // let (path, max_d) = ("input_example.txt", 32);

    let points: HashSet<Point> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            let (col, row) = line.split_once(", ").unwrap();
            Point {
                row: row.parse::<u32>().unwrap(),
                col: col.parse::<u32>().unwrap(),
            }
        })
        .collect();

    let width = points.iter().max_by_key(|pt| pt.col).unwrap().col;
    let height = points.iter().max_by_key(|pt| pt.row).unwrap().row;

    let mut to_exclude: HashSet<Point> = HashSet::new();
    let mut grid: HashMap<Point, Point> = HashMap::new();

    for row in 0..=height {
        for col in 0..=width {
            let current = Point { row, col };

            // (Source, distance)
            let distances: Vec<(Point, u32)> = points
                .iter()
                .map(|pt| (*pt, row.abs_diff(pt.row) + col.abs_diff(pt.col)))
                .collect();

            let (source, distance_mini) = distances.iter().min_by_key(|(_, d)| *d).unwrap();
            if distances.iter().filter(|(_, d)| d == distance_mini).count() == 1 {
                grid.insert(current, *source);

                if row == 0 || col == 0 || row == height || col == width {
                    to_exclude.insert(*source);
                }
            }
        }
    }

    let finite_areas: HashMap<Point, u32> = points
        .difference(&to_exclude)
        .map(|source| {
            (
                *source,
                grid.iter().filter(|(_, s)| *s == source).count() as u32,
            )
        })
        .collect();

    // print(height, width, &points, &grid, &to_exclude);
    println!("Part one: {}", finite_areas.values().max().unwrap());

    // --
    let mut area_size = 0;
    for row in 0..=height {
        for col in 0..=width {
            let current = Point { row, col };

            if points
                .iter()
                .map(|pt| pt.row.abs_diff(current.row) + pt.col.abs_diff(current.col))
                .sum::<u32>()
                < max_d
            {
                area_size += 1;
            }
        }
    }

    println!("Part two: {}", area_size);
}

#[allow(dead_code)]
fn print(
    height: u32,
    width: u32,
    points: &HashSet<Point>,
    grid: &HashMap<Point, Point>,
    to_exclude: &HashSet<Point>,
) {
    for row in 0..=height {
        for col in 0..=width {
            let point = Point { row, col };

            if points.contains(&point) {
                print!("S");
            } else if let Some(source) = grid.get(&point) {
                if !to_exclude.contains(source) {
                    print!("o");
                } else {
                    print!(".");
                }
            } else {
                print!(".");
            }
        }
        println!();
    }
}
