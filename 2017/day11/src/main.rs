// https://www.redblobgames.com/grids/hexagons/#neighbors
// Coordinates:  q, r, s
//         and:  q + r + s always = 0

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let mut current: (i32, i32, i32) = (0, 0, 0);

    let input = std::fs::read_to_string(path).unwrap();
    let mut maxi = 0;
    for dir in input.trim().split(',') {
        current = step(current, dir);
        maxi = maxi.max(distance(&current));
    }

    println!("Part one: {}", distance(&current));
    println!("Part two: {}", maxi);
}

fn distance(current: &(i32, i32, i32)) -> i32 {
    current.0.abs().max(current.1.abs()).max(current.2.abs())
}

fn step(current: (i32, i32, i32), direction: &str) -> (i32, i32, i32) {
    match direction {
        "n" => (current.0 + 1, current.1, current.2 - 1),
        "ne" => (current.0, current.1 + 1, current.2 - 1),
        "se" => (current.0 - 1, current.1 + 1, current.2),
        "s" => (current.0 - 1, current.1, current.2 + 1),
        "sw" => (current.0, current.1 - 1, current.2 + 1),
        _ => (current.0 + 1, current.1 - 1, current.2),
    }
}
