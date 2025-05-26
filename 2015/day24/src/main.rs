use std::{cmp::Ordering, collections::HashSet};

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let input: Vec<usize> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect();

    println!(
        "Part one: {}",
        get_best_entanglement(&input, input.iter().sum::<usize>() / 3)
    );
    println!(
        "Part two: {}",
        get_best_entanglement(&input, input.iter().sum::<usize>() / 4)
    );
}

fn get_best_entanglement(input: &[usize], package_size: usize) -> usize {
    let packages = get_packages(input, Vec::new(), package_size);
    let smallest_length = packages.iter().min_by_key(|g| g.len()).unwrap().len();

    *packages
        .iter()
        .filter(|g| g.len() == smallest_length)
        .map(|g| g.iter().product::<usize>())
        .collect::<Vec<usize>>()
        .iter()
        .min()
        .unwrap()
}

fn get_packages(input: &[usize], current: Vec<usize>, package_size: usize) -> HashSet<Vec<usize>> {
    let mut packages = HashSet::new();

    for (i, val) in input.iter().enumerate() {
        let mut current = current.clone();
        current.push(*val);

        match current.iter().sum::<usize>().cmp(&package_size) {
            Ordering::Less => {
                packages.extend(get_packages(&input[i + 1..], current, package_size));
            }
            Ordering::Equal => {
                packages.insert(current);
            }
            Ordering::Greater => {}
        }
    }
    packages
}
