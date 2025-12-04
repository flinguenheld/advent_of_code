use std::collections::HashSet;

use anyhow::Result;

const NEIGHBOURS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, -1),
    (0, 1),
];

fn main() -> Result<()> {
    let path = "input.txt";
    // let path = "input_example.txt";

    let mut rolls: HashSet<(i32, i32)> = std::fs::read_to_string(path)?
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.char_indices()
                .filter(|(_, c)| *c == '@')
                .map(move |(col, _)| (row as i32, col as i32))
        })
        .collect();

    println!(
        "Part one: {}",
        rolls.iter().fold(0, |acc, roll| {
            acc + match is_removable_roll(&rolls, roll) {
                true => 1,
                false => 0,
            }
        })
    );

    // --
    let mut amount_removed = 0;
    let mut removed: HashSet<(i32, i32)> = HashSet::new();
    while !removed.is_empty() || amount_removed == 0 {
        removed.clear();
        rolls.iter().for_each(|roll| {
            if is_removable_roll(&rolls, roll) {
                removed.insert(*roll);
                amount_removed += 1;
            }
        });

        rolls.retain(|roll| !removed.contains(roll));
    }
    println!("Part two: {}", amount_removed);

    Ok(())
}

fn is_removable_roll(rolls: &HashSet<(i32, i32)>, roll: &(i32, i32)) -> bool {
    // . . .
    // . o .
    // . . .

    NEIGHBOURS
        .map(|n| (roll.0 + n.0, roll.1 + n.1))
        .iter()
        .fold(0, |acc_n, n| {
            acc_n
                + match rolls.contains(n) {
                    true => 1,
                    false => 0,
                }
        })
        < 4
}

#[allow(dead_code)]
fn print_rolls(rolls: &HashSet<(i32, i32)>) -> Result<()> {
    let row_max = rolls.iter().max_by_key(|(row, _)| row).unwrap().0;
    let col_max = rolls.iter().max_by_key(|(_, col)| col).unwrap().1;

    for row in 0..=row_max {
        for col in 0..=col_max {
            if rolls.contains(&(row, col)) {
                print!("@");
            } else {
                print!(".");
            }
        }
        println!();
    }

    Ok(())
}
