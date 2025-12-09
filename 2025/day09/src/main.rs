use std::cmp;

use anyhow::Result;

fn main() -> Result<()> {
    let path = "input.txt";
    // let path = "input_example.txt";

    let red_tiles: Vec<(i64, i64)> = std::fs::read_to_string(path)?
        .lines()
        .map(|line| {
            let (col, row) = line.trim().split_once(',').unwrap();

            let row = row.parse::<i64>().unwrap();
            let col = col.parse::<i64>().unwrap();

            (row, col)
        })
        .collect();

    println!("{:?}", &red_tiles);

    let mut max = 0;
    for (i, one) in red_tiles.iter().enumerate() {
        for two in red_tiles.iter().skip(i) {
            let area = (one.0 - two.0 + 1).abs() * (one.1 - two.1 + 1).abs();
            println!("area: {}     {:?} - {:?}", area, one, two);
            max = cmp::max(max, area);
        }
    }

    print_floor(&red_tiles);
    println!("Part one: {}", max);
    Ok(())
}

fn print_floor(red_tiles: &[(i64, i64)]) {
    let max_row = red_tiles.iter().max_by_key(|(row, _)| row).unwrap().0 + 1;
    let max_col = red_tiles.iter().max_by_key(|(_, col)| col).unwrap().1 + 1;

    for row in 0..=max_row {
        for col in 0..=max_col {
            if red_tiles.contains(&(row, col)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
