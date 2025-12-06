use anyhow::Result;

fn main() -> Result<()> {
    let path = "input.txt";
    // let path = "input_example.txt";

    let input = std::fs::read_to_string(path)?;

    let mut operators: Vec<(char, usize)> = Vec::new();
    for c in input
        .lines()
        .find(|line| line.contains('+'))
        .unwrap()
        .chars()
    {
        if c != ' ' {
            operators.push((c, 0));
        } else {
            operators.last_mut().unwrap().1 += 1;
        }
    }
    operators.last_mut().unwrap().1 += 1;

    // --
    let mut values: Vec<Vec<String>> = Vec::new();
    for line in std::fs::read_to_string(path)?
        .lines()
        .filter(|line| !line.contains('+'))
    {
        let mut from = 0;
        for (col, (_, len)) in operators.iter().enumerate() {
            if let Some(row) = values.get_mut(col) {
                row.push(line[from..from + len].to_string());
            } else {
                values.push(vec![line[from..from + len].to_string()]);
            }
            from += len + 1;
        }
    }
    println!("Part one: {}", calculate(&values, &operators));

    // --
    let mut new_values: Vec<Vec<String>> = Vec::new();
    for (row, (_, len)) in values.iter().zip(operators.iter()) {
        new_values.push(Vec::new());
        for i in 0..*len {
            let mut new_value = String::new();
            for value in row.iter() {
                new_value.push(value.chars().nth(i).unwrap());
            }
            new_values.last_mut().unwrap().push(new_value);
        }
    }
    println!("Part two: {}", calculate(&new_values, &operators));
    Ok(())
}

fn calculate(values: &[Vec<String>], operators: &[(char, usize)]) -> u64 {
    let mut total = 0;
    for (row, (operator, _)) in values.iter().zip(operators.iter()) {
        if *operator == '+' {
            total += row.iter().fold(0, |acc, val| acc + to_int(val));
        } else {
            total += row.iter().fold(1, |acc, val| acc * to_int(val));
        }
    }
    total
}

fn to_int(value: &str) -> u64 {
    value
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .unwrap()
}
