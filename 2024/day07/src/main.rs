use std::{collections::HashMap, fs};

fn main() {
    // let path = "input_example.txt";
    let path = "input.txt";

    let input = fs::read_to_string(path).unwrap();
    let mut input: HashMap<i64, Vec<i64>> = input
        .lines()
        .map(|line| {
            let (res, values) = line.split_once(':').unwrap();

            (
                res.parse::<i64>().unwrap(),
                values
                    .split_whitespace()
                    .rev()
                    .map(|v| v.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>(),
            )
        })
        .collect();

    // Part one --
    let mut tot = 0;
    for (goal, values) in input.clone().iter_mut() {
        let mut ok = false;
        calcul(*goal, values.pop().unwrap(), values.clone(), &mut ok, false);
        if ok {
            tot += goal;
        }
    }
    println!("Part one: {}", tot);

    // Part two --
    let mut tot = 0;
    for (goal, values) in input.iter_mut() {
        let mut ok = false;
        calcul(*goal, values.pop().unwrap(), values.clone(), &mut ok, true);
        if ok {
            tot += goal;
        }
    }
    println!("Part two: {}", tot);
}

fn calcul(goal: i64, current: i64, mut fields: Vec<i64>, is_ok: &mut bool, concat: bool) {
    if let Some(left) = fields.pop() {
        if !(*is_ok) {
            calcul(goal, current + left, fields.clone(), is_ok, concat);
            calcul(goal, current * left, fields.clone(), is_ok, concat);

            if concat {
                let a = left.to_string();
                let b = current.to_string();
                calcul(
                    goal,
                    (b + &a).parse::<i64>().unwrap(),
                    fields.clone(),
                    is_ok,
                    true,
                );
            }
        }
    } else if current == goal {
        *is_ok = true;
    }
}
