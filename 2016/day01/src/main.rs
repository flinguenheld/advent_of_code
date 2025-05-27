fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let movements = [
        |(_, y): &mut (i32, i32)| *y += 1,
        |(x, _): &mut (i32, i32)| *x += 1,
        |(_, y): &mut (i32, i32)| *y -= 1,
        |(x, _): &mut (i32, i32)| *x -= 1,
    ];

    let mut index: usize = 0;
    let mut position = (0, 0);

    let mut part_two = -1;
    let mut locations = vec![(0, 0)];

    for movement in std::fs::read_to_string(path).unwrap().split(", ") {
        let steps = movement
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<i32>()
            .unwrap();

        index = match movement.chars().next().unwrap() {
            'R' => index as i8 + 1,
            _ => index as i8 - 1,
        }
        .rem_euclid(4) as usize;

        for _ in 0..steps {
            movements[index](&mut position);

            if part_two == -1 && locations.contains(&position) {
                part_two = position.0.abs() + position.1.abs();
            }
            locations.push(position);
        }
    }
    println!("Part one: {}", position.0.abs() + position.1.abs());
    println!("Part two: {}", part_two);
}
