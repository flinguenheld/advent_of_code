fn main() {
    let (path, liters) = ("input.txt", 150);
    // let (path, liters) = ("input_example.txt", 25);

    let containers: Vec<i32> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .collect();

    let valids = go_list(&containers, Vec::new(), liters);

    println!("Part one: {}", valids.len());

    let min_lenght = valids.iter().min_by_key(|c| c.len()).unwrap().len();
    println!(
        "Part two: {}",
        valids.iter().filter(|c| c.len() == min_lenght).count()
    );
}

fn go_list(containers: &[i32], current: Vec<i32>, to_reach: i32) -> Vec<Vec<i32>> {
    {
        if current.iter().sum::<i32>() == to_reach {
            return vec![current];
        } else if current.iter().sum::<i32>() > to_reach || containers.is_empty() {
            return Vec::new();
        }

        let mut valid_combinations: Vec<Vec<i32>> = Vec::new();
        for (i, c) in containers.iter().enumerate() {
            let mut new_current = current.clone();
            new_current.push(*c);

            let mut ok = go_list(&containers[i + 1..], new_current, to_reach);
            if !ok.is_empty() {
                valid_combinations.append(&mut ok);
            }
        }
        valid_combinations
    }
}
