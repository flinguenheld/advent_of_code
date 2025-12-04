use anyhow::Result;

fn main() -> Result<()> {
    let path = "input.txt";
    // let path = "input_example.txt";

    let input: Vec<Vec<u8>> = std::fs::read_to_string(path)?
        .lines()
        .map(|line| line.chars().map(|c| c as u8 - b'0').collect())
        .collect();

    println!(
        "Part one: {}",
        input.iter().fold(0, |acc, line| acc + next_max(line, 2))
    );
    println!(
        "Part two: {}",
        input.iter().fold(0, |acc, line| acc + next_max(line, 12))
    );
    Ok(())
}

fn next_max(line: &[u8], range: usize) -> u64 {
    if range == 0 {
        return 0;
    }

    let (mut max, mut position) = (0, 0);
    for (i, v) in line.iter().enumerate().take(line.len() - (range - 1)) {
        if *v > max {
            (position, max) = (i, *v);
        }
    }
    max as u64 * 10_u64.pow(range as u32 - 1) + next_max(&line[position + 1..], range - 1)
}
