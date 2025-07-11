// const COORDINATES=[(0,0), ()]

use std::collections::HashMap;

fn main() {
    // let input = 18;
    // let input = 42;
    let input = 1723;

    let mut grid = vec![vec![0; 300]; 300];

    for (y, row) in grid.iter_mut().enumerate() {
        for (x, case) in row.iter_mut().enumerate() {
            *case = power_level(x + 1, y + 1, input);
        }
    }

    // print(&grid);

    let mut cache: HashMap<(usize, usize), i32> = HashMap::new();
    let square_size = 3;
    let grid_size = 300;

    for y in 0..(grid_size - square_size) {
        for x in 0..(grid_size - square_size) {
            let mut total = 0;
            for i in 0..square_size {
                for j in 0..square_size {
                    total += grid[y + i][x + j];
                }
            }

            cache.insert((x + 1, y + 1), total);
        }
    }

    let coordinates = cache.iter().max_by_key(|(_, v)| **v).unwrap().0;
    println!("Part one: {},{}", coordinates.0, coordinates.1);

    // --
    let mut cache: HashMap<(usize, usize, usize), i32> = HashMap::new();
    // let mut prev = 0;
    for y in 0..grid_size {
        for x in 0..grid_size {
            'get_squares: for square_size in 1..=(grid_size - x.max(y)) {
                let mut total = 0;
                for i in (0..square_size).rev() {
                    for j in (0..square_size).rev() {
                        total += grid[y + i][x + j];
                    }
                }

                cache.insert((x + 1, y + 1, square_size), total);
                if total < -10 {
                    break 'get_squares;
                }
            }
        }
    }
    let coordinates = cache.iter().max_by_key(|(_, v)| **v).unwrap().0;
    println!(
        "Part two: {},{},{}",
        coordinates.0, coordinates.1, coordinates.2
    );
}

#[allow(dead_code)]
fn print(grid: &[Vec<i32>]) {
    for row in grid[..30].iter() {
        for col in row[..30].iter() {
            print!("{:3}", col)
        }
        println!();
    }
}

fn power_level(x: usize, y: usize, input: i32) -> i32 {
    let rack_id = x as i32 + 10;
    let mut power = rack_id * y as i32;
    power += input;
    power *= rack_id;
    power %= 1000;
    power /= 100;
    power -= 5;
    power
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_level() {
        assert_eq!(power_level(3, 5, 8), 4);
        assert_eq!(power_level(122, 79, 57), -5);
        assert_eq!(power_level(217, 196, 39), 0);
        assert_eq!(power_level(101, 153, 71), 4);
    }
}
