use std::fs;

#[derive(Clone, Copy)]
struct Node {
    value: u64,
    paths_end: u64,
}
impl Node {
    fn new(value: u64) -> Self {
        Node {
            value,
            paths_end: 1,
        }
    }
}

fn main() {
    // let path = "input_example.txt";
    // let path = "input_example_2.txt";
    let path = "input.txt";

    let mut input: Vec<Node> = fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| Node::new(l.parse::<u64>().unwrap()))
        .collect();
    input.sort_by_key(|n| n.value);

    let (acc1, acc3) = input.windows(2).fold((1, 1), |(acc1, acc3), win| {
        match win[1].value - win[0].value {
            1 => (acc1 + 1, acc3),
            3 => (acc1, acc3 + 1),
            _ => (acc1, acc3),
        }
    });
    println!("Part one: {}", acc1 * acc3);

    // --
    // Each nodes have a default number of "path to the end" to 1.
    // For each nodes (from back), check if some previous are optionals.
    // If so, save the sum of these available jumps.
    // Otherwise simply add the direct previous node "path to the end".
    input.insert(0, Node::new(0));
    input.push(Node::new(input.last().unwrap().value + 3));

    for i in (0..input.len()).rev() {
        let a = input[i];

        if let Some(b) = input.get(i + 1) {
            let c = input.get(i + 2);
            let d = input.get(i + 3);

            if d.is_some() && d.unwrap().value - a.value <= 3 {
                input[i].paths_end = b.paths_end + c.unwrap().paths_end + d.unwrap().paths_end;
            } else if c.is_some() && c.unwrap().value - a.value <= 3 {
                input[i].paths_end = b.paths_end + c.unwrap().paths_end;
            } else {
                input[i].paths_end = b.paths_end;
            }
        }
    }

    println!("Part two: {}", input[0].paths_end);
}
