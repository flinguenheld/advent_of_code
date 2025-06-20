#[derive(Debug, Clone)]
struct Program {
    name: String,
    weight: u32,
    hold: Vec<String>,
}

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let input = std::fs::read_to_string(path).unwrap();
    let programs: Vec<Program> = input
        .lines()
        .map(|line| {
            let fields: Vec<&str> = line.split_whitespace().collect();

            Program {
                name: fields[0].to_string(),
                weight: fields[1]
                    .chars()
                    .filter(|c| c.is_ascii_digit())
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap(),
                hold: fields
                    .iter()
                    .skip(3)
                    .map(|h| {
                        h.chars()
                            .filter(|c| c.is_ascii_alphabetic())
                            .collect::<String>()
                    })
                    .collect(),
            }
        })
        .collect();

    let all_held: Vec<String> = programs
        .iter()
        .filter(|p| !p.hold.is_empty())
        .flat_map(|p| p.hold.clone())
        .collect();

    let mut master = programs
        .iter()
        .find(|p| !all_held.contains(&p.name))
        .unwrap()
        .name
        .clone();

    println!("Part one: {:?}", master);

    // --
    // Find the wrong list
    // And dig until it becomes correct
    let mut prev_group = Vec::new();

    loop {
        let currents = programs
            .iter()
            .find(|p| p.name == master)
            .unwrap()
            .hold
            .clone();

        let group: Vec<(Program, u32)> = currents
            .iter()
            .map(|c| {
                (
                    programs.iter().find(|p| p.name == *c).unwrap().clone(),
                    sum(c.as_str(), &programs),
                )
            })
            .collect();

        // Is there a wrong group ?
        // If so, keep diging!
        if let Some(alone) = group
            .iter()
            .find(|prog| group.iter().filter(|p| p.1 == prog.1).count() == 1)
        {
            master = alone.0.name.clone();
            prev_group = group;
        } else {
            // Here
            // So get the alone and the regular value
            let alone = prev_group
                .iter()
                .find(|prog| prev_group.iter().filter(|p| p.1 == prog.1).count() == 1)
                .unwrap();
            let regular = prev_group.iter().find(|p| p.1 != alone.1).unwrap();

            // Tada
            println!("Part two: {}", alone.0.weight + regular.1 - alone.1);
            break;
        }
    }
}

fn sum(who: &str, programs: &[Program]) -> u32 {
    let mut total = 0;

    let prog = programs.iter().find(|p| p.name == who).unwrap();
    total += prog.weight;

    for p in prog.hold.iter() {
        total += sum(p.as_str(), programs);
    }

    total
}
