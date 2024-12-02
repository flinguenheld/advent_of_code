use std::fs;

fn main() {
    // let path = "input_01_example.txt";
    let path = "input_01.txt";

    let input = fs::read_to_string(path).unwrap();
    let input: Vec<Vec<i32>> = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|val| val.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    // Part one --
    let count = input.iter().filter(|report| is_ok(report)).count();
    println!("Part one : {}", count);

    // Part two --
    let mut count = 0;
    'blah: for report in input.iter() {
        for i in 0..report.len() {
            let mut temp = report.clone();
            temp.remove(i);

            if is_ok(&temp) {
                count += 1;
                continue 'blah;
            }
        }
    }

    println!("Part two : {}", count);
}

fn is_ok(report: &[i32]) -> bool {
    let jumps: Vec<i32> = report.windows(2).map(|w| w[0] - w[1]).collect();
    jumps.iter().all(|v| *v < 0 && *v >= -3) || jumps.iter().all(|v| *v > 0 && *v <= 3)
}
