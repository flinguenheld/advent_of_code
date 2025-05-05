use std::fs;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct Field {
    name: String,
    from_a: u64,
    to_a: u64,
    from_b: u64,
    to_b: u64,
}

fn main() {
    // let path = "input_example.txt";
    // let path = "input_example_2.txt";
    let path = "input.txt";

    let mut fields: Vec<Field> = Vec::new();
    let mut my_ticket: Vec<u64> = Vec::new();
    let mut valid_tickets: Vec<Vec<u64>> = Vec::new();

    let mut total_p1 = 0;

    for (i, group) in fs::read_to_string(path).unwrap().split("\n\n").enumerate() {
        if i == 0 {
            fields = group
                .lines()
                .map(|l| {
                    let (name, ranges) = l.split_once(": ").unwrap();
                    let (a, b) = ranges.split_once(" or ").unwrap();
                    let (from_a, to_a) = a.split_once('-').unwrap();
                    let (from_b, to_b) = b.split_once('-').unwrap();

                    Field {
                        name: name.to_string(),
                        from_a: from_a.parse::<u64>().unwrap(),
                        to_a: to_a.parse::<u64>().unwrap(),
                        from_b: from_b.parse::<u64>().unwrap(),
                        to_b: to_b.parse::<u64>().unwrap(),
                    }
                })
                .collect();
        } else if i == 1 {
            my_ticket = group[13..]
                .split(',')
                .map(|v| v.parse::<u64>().unwrap())
                .collect();
        } else {
            for nearby in group[16..].lines() {
                let values = nearby
                    .split(',')
                    .map(|v| v.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();

                if let Some(fail) = values.iter().find(|v| {
                    !fields
                        .iter()
                        .any(|f| (f.from_a..=f.to_a).contains(v) || (f.from_b..=f.to_b).contains(v))
                }) {
                    total_p1 += fail;
                } else {
                    valid_tickets.push(values);
                }
            }
        }
    }

    println!("Part one: {}", total_p1);

    // --
    // Create a vec of fields with all fields.
    // Then, per vector, remove all fields which do not fit the values.
    // Those vectors where there're only one of them are correct,
    // So remove them if they are still in the other vectors.
    // And do that until all vec contain only one value
    valid_tickets.push(my_ticket.clone());
    let mut sorted_fields: Vec<Vec<Field>> = (0..fields.len()).map(|_| fields.clone()).collect();

    for (i, field) in sorted_fields.iter_mut().enumerate() {
        let values_to_valid: Vec<u64> = valid_tickets.iter().map(|t| t[i]).collect();

        field.retain(|f| {
            values_to_valid
                .iter()
                .all(|v| (f.from_a..=f.to_a).contains(v) || (f.from_b..=f.to_b).contains(v))
        });
    }

    while sorted_fields.iter().any(|sf| sf.len() > 1) {
        let validated: Vec<Field> = sorted_fields
            .iter()
            .filter(|f| f.len() == 1)
            .map(|f| f[0].clone())
            .collect();

        sorted_fields
            .iter_mut()
            .filter(|sf| sf.len() > 1)
            .for_each(|field| field.retain(|f| !validated.contains(f)))
    }

    println!(
        "Part two: {}",
        sorted_fields
            .iter()
            .zip(my_ticket.iter())
            .fold(1, |acc, (f, t)| {
                match f[0].name.starts_with("departure") {
                    true => acc * t,
                    false => acc,
                }
            })
    );
}
