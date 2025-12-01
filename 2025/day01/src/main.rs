use anyhow::{Context, Result};

fn main() -> Result<()> {
    let path = "input.txt";
    // let path = "input_example.txt";

    let input = std::fs::read_to_string(path).context("Wrong path")?;

    let mut p1 = 0;
    let mut p2 = 0;
    let mut number = 50;
    for line in input.lines() {
        let direction = match line.starts_with('R') {
            true => 1,
            false => -1,
        };
        for _ in 0..line[1..].parse()? {
            number += direction;
            p2 += (number % 100 == 0) as i32;
        }
        p1 += (number % 100 == 0) as i32;
    }
    println!("Part one {}", p1);
    println!("Part two {}", p2);
    Ok(())
}
