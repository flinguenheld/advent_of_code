use std::collections::VecDeque;

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let cavern: Vec<Vec<u32>> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| line.chars().map(|c| c as u32 - 48).collect())
        .collect();

    println!("Part one: {}", get_min_risk(&cavern));

    // --
    let mut big_cavern = cavern.clone();

    for time in 1..5 {
        for (index, original_row) in cavern.iter().enumerate() {
            big_cavern[index]
                .append(&mut original_row.iter().map(|v| bigcave_val(v + time)).collect());
        }
    }

    for time in 1..5 {
        for i in 0..cavern.len() {
            big_cavern.push(
                big_cavern[i]
                    .iter()
                    .map(|v| bigcave_val(v + time))
                    .collect(),
            );
        }
    }

    println!("Part two: {}", get_min_risk(&big_cavern));
}

fn bigcave_val(value: u32) -> u32 {
    if value <= 9 {
        value
    } else if value % 9 == 0 {
        1
    } else {
        value % 9
    }
}

fn get_min_risk(cavern: &[Vec<u32>]) -> u32 {
    let mut cache: Vec<Vec<u32>> = cavern.into();
    cache.iter_mut().flatten().for_each(|v| *v = u32::MAX);
    cache[0][0] = 0;

    // dbg!(&cavern);

    let mut queue = VecDeque::from([(0, 0)]);

    while let Some(current_pos) = queue.pop_front() {
        for (row, col) in get_neighbours(current_pos, cavern).iter() {
            if cache[*row][*col] == u32::MAX
                || cache[*row][*col] > cache[current_pos.0][current_pos.1] + cavern[*row][*col]
            {
                cache[*row][*col] = cache[current_pos.0][current_pos.1] + cavern[*row][*col];
                queue.push_back((*row, *col));
            }
        }
    }

    cache[cache.len() - 1][cache[0].len() - 1]
}

fn get_neighbours(point: (usize, usize), cavern: &[Vec<u32>]) -> VecDeque<(usize, usize)> {
    let mut n = VecDeque::new();

    if point.0 > 0 {
        n.push_back((point.0 - 1, point.1));
    }
    if point.0 < cavern.len() - 1 {
        n.push_back((point.0 + 1, point.1));
    }
    if point.1 > 0 {
        n.push_back((point.0, point.1 - 1));
    }
    if point.1 < cavern[0].len() - 1 {
        n.push_back((point.0, point.1 + 1));
    }

    n
}
