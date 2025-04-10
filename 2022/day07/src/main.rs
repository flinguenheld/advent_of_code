use std::{collections::HashMap, fs, path::PathBuf, str::FromStr};

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let input = fs::read_to_string(path).unwrap();

    let mut folders: HashMap<PathBuf, u32> = HashMap::new();
    let mut current: PathBuf = PathBuf::new();

    // Only save paths and sizes, the rest is a trap
    // And update all parent for each file on the fly
    for line in input.lines() {
        if line == "$ cd .." {
            current.pop();
        } else if let Some(dirname) = line.strip_prefix("$ cd ") {
            current.push(dirname);
        } else if let Some((size, _)) = line.split_once(' ') {
            if let Ok(size) = size.parse::<u32>() {
                let mut cur = current.clone();

                loop {
                    folders
                        .entry(cur.clone())
                        .and_modify(|v| *v += size)
                        .or_insert(size);

                    if !cur.pop() {
                        break;
                    }
                }
            }
        }
    }

    println!(
        "Part one: {}",
        folders
            .iter()
            .filter_map(|(_, size)| match *size <= 100_000 {
                true => Some(size),
                false => None,
            })
            .sum::<u32>()
    );

    // --
    let total = (folders.get(&PathBuf::from_str("/").unwrap())).unwrap();
    let need = 30_000_000 - (70_000_000 - total);

    println!(
        "Part two: {}",
        folders
            .iter()
            .filter_map(|(_, size)| match *size > need {
                true => Some(size),
                false => None,
            })
            .min()
            .unwrap()
    );
}
