use std::{collections::HashSet, fs};

#[derive(PartialEq, PartialOrd, Ord, Eq, Hash, Debug)]
struct Coordinate {
    row: i32,
    col: i32,
    z: i32,
    w: i32,
}

fn main() {
    // let path = "input_example.txt";
    let path = "input.txt";

    let mut map: HashSet<Coordinate> = HashSet::new();
    for (row, line) in fs::read_to_string(path).unwrap().lines().enumerate() {
        for (col, case) in line.char_indices() {
            if case == '#' {
                map.insert(Coordinate {
                    row: col as i32,
                    col: row as i32,
                    z: 0,
                    w: 0,
                });
            }
        }
    }

    for _ in 0..6 {
        let min_col = map.iter().min_by_key(|c| c.col).unwrap().col;
        let max_col = map.iter().max_by_key(|c| c.col).unwrap().col;

        let min_row = map.iter().min_by_key(|c| c.row).unwrap().row;
        let max_row = map.iter().max_by_key(|c| c.row).unwrap().row;

        let min_z = map.iter().min_by_key(|c| c.z).unwrap().z;
        let max_z = map.iter().max_by_key(|c| c.z).unwrap().z;

        let min_w = map.iter().min_by_key(|c| c.w).unwrap().w;
        let max_w = map.iter().max_by_key(|c| c.w).unwrap().w;

        let mut to_add = HashSet::new();
        let mut to_remove = HashSet::new();

        for w in (min_w - 1)..=(max_w + 1) {
            for z in (min_z - 1)..=(max_z + 1) {
                for row in (min_row - 1)..=(max_row + 1) {
                    for col in (min_col - 1)..=(max_col + 1) {
                        let coordinate = Coordinate { row, col, z, w };
                        let neighbors = map
                            .iter()
                            .filter(|c| {
                                **c != coordinate
                                    && (c.row >= row - 1 && c.row <= row + 1)
                                    && (c.col >= col - 1 && c.col <= col + 1)
                                    && (c.z >= z - 1 && c.z <= z + 1)
                                    && (c.w >= w - 1 && c.w <= w + 1)
                            })
                            .count();

                        if map.contains(&coordinate) {
                            if !(neighbors == 2 || neighbors == 3) {
                                to_remove.insert(coordinate);
                            }
                        } else if neighbors == 3 {
                            to_add.insert(coordinate);
                        }
                    }
                }
            }
        }

        map.retain(|c| !to_remove.contains(c));
        map.extend(to_add);
    }

    println!("Part two: {}", map.len());
}
