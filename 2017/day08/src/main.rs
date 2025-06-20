use std::collections::HashMap;

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let mut registers: HashMap<&str, i32> = HashMap::new();
    let mut maxi = 0;

    let input = std::fs::read_to_string(path).unwrap();
    for line in input.lines() {
        let fields: Vec<&str> = line.split_whitespace().collect();

        let to_check = registers.entry(fields[4]).or_insert(0);
        let cmp = fields[6].parse::<i32>().unwrap();

        let ok = match fields[5] {
            "==" => *to_check == cmp,
            "!=" => *to_check != cmp,
            "<=" => *to_check <= cmp,
            "<" => *to_check < cmp,
            ">=" => *to_check >= cmp,
            _ => *to_check > cmp,
        };

        if ok {
            let value = fields[2].parse::<i32>().unwrap();
            if fields[1] == "inc" {
                registers
                    .entry(fields[0])
                    .and_modify(|e| *e += value)
                    .or_insert(value);
            } else {
                registers
                    .entry(fields[0])
                    .and_modify(|e| *e -= value)
                    .or_insert(-value);
            }
        }

        maxi = std::cmp::max(maxi, *registers.entry(fields[0]).or_default());
    }

    println!("Part one: {}", registers.values().max().unwrap());
    println!("Part two: {}", maxi);
}
