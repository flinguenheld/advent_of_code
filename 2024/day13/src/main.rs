use std::fs;

#[derive(Debug)]
struct Equation {
    a_x: i64,
    b_x: i64,
    x_sum: i64,

    a_y: i64,
    b_y: i64,
    y_sum: i64,
}

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let input = fs::read_to_string(path).unwrap();
    let input: Vec<&str> = input.lines().filter(|line| !line.is_empty()).collect();

    let input: Vec<Equation> = input
        .windows(3)
        .step_by(3)
        .map(|bloc| {
            let line0 = extract_numbers(bloc[0]);
            let line1 = extract_numbers(bloc[1]);
            let line2 = extract_numbers(bloc[2]);

            // cargo run -F part_one
            #[cfg(feature = "part_one")]
            let eq = Equation {
                a_x: line0.0,
                b_x: line1.0,
                x_sum: line2.0,
                a_y: line0.1,
                b_y: line1.1,
                y_sum: line2.1,
            };

            #[cfg(not(feature = "part_one"))]
            let eq = Equation {
                a_x: line0.0,
                b_x: line1.0,
                x_sum: line2.0 + 10000000000000,
                a_y: line0.1,
                b_y: line1.1,
                y_sum: line2.1 + 10000000000000,
            };

            eq
        })
        .collect();

    let mut total = 0;
    for equation in input.iter() {
        // println!("Equation {:?}", equation);

        let a = ((equation.x_sum * equation.b_y) - (equation.y_sum * equation.b_x))
            / ((equation.a_x * equation.b_y) - (equation.a_y * equation.b_x));

        let b = (equation.y_sum - a * equation.a_y) / equation.b_y;

        if (equation.a_x * a) + (equation.b_x * b) == equation.x_sum
            && (equation.a_y * a) + (equation.b_y * b) == equation.y_sum
        {
            // println!("this one is ok");
            // println!("A: {}", a);
            // println!("B: {}", b);
            total += 3 * a + b;
        }
    }

    #[cfg(feature = "part_one")]
    println!("Part one: {}", total);

    #[cfg(not(feature = "part_one"))]
    println!("Part two: {}", total);
}

fn extract_numbers(input: &str) -> (i64, i64) {
    let (x, y) = input.split_once(',').unwrap();

    let x = x.chars().filter(|c| c.is_ascii_digit()).collect::<String>();
    let y = y.chars().filter(|c| c.is_ascii_digit()).collect::<String>();

    (x.parse().unwrap(), y.parse().unwrap())
}
