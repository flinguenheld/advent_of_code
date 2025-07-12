use std::collections::HashSet;

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let input = std::fs::read_to_string(path).unwrap();
    let mut plants: HashSet<i64> = HashSet::new();
    for (i, c) in input.lines().next().unwrap().chars().skip(15).enumerate() {
        if c == '#' {
            plants.insert(i as i64);
        }
    }

    let rules: HashSet<Vec<i64>> = input
        .lines()
        .skip(2)
        .filter(|l| l.ends_with('#'))
        .map(|line| {
            let (left, _) = line.split_once(" => ").unwrap();

            left.char_indices()
                .filter(|(_, c)| *c == '#')
                .map(|(i, _)| i as i64 - 2)
                .collect()
        })
        .collect();

    // The pattern is stable after 90, so:
    // Get the value at 100
    // Get the value at 200
    //
    // And simple maths to find 50 billons -_-

    let mut a = 0;
    let mut b = 0;

    for i in 1..=200 {
        plants = proceed(plants, &rules);
        // print(&plants);

        if i == 20 {
            println!("Part one: {}", plants.iter().sum::<i64>());
        } else if i == 100 {
            a = plants.iter().sum::<i64>();
        } else if i == 200 {
            b = plants.iter().sum::<i64>();
        }
    }

    let mul = (50_000_000_000_i64 - 100) / 100;
    let result = (mul * (b - a)) + a;

    println!("Part two: {}", result);
}

fn proceed(plants: HashSet<i64>, rules: &HashSet<Vec<i64>>) -> HashSet<i64> {
    let mut temp: HashSet<i64> = HashSet::new();

    let min = *plants.iter().min().unwrap() - 2;
    let max = *plants.iter().max().unwrap() + 2;

    for index in min..=max {
        let k: Vec<i64> = (-2..=2).filter(|i| plants.contains(&(index + i))).collect();
        // println!("index: {}  -> k {:?}", index, k);

        if rules.contains(&k) {
            temp.insert(index);
        }
    }

    temp
}

#[allow(dead_code)]
fn print(plants: &HashSet<i64>) {
    let min = *plants.iter().min().unwrap();
    let max = *plants.iter().max().unwrap();

    for i in min..=max {
        if plants.contains(&i) {
            print!("#");
        } else {
            print!(".");
        }
    }
    println!();
}
