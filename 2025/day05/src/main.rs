use anyhow::Result;
use std::cmp;

#[derive(Debug, Clone, Copy)]
struct Range {
    from: u64,
    to: u64,
}

fn main() -> Result<()> {
    let path = "input.txt";
    // let path = "input_example.txt";

    let mut ranges: Vec<Range> = std::fs::read_to_string(path)?
        .lines()
        .filter(|l| l.contains('-'))
        .map(|line| {
            let (from, to) = line.split_once('-').unwrap();
            Range {
                from: from.parse::<u64>().unwrap(),
                to: to.parse::<u64>().unwrap(),
            }
        })
        .collect();

    ranges.sort_by_key(|range| range.to);
    ranges.sort_by_key(|range| range.from);

    'again: loop {
        for i in 0..ranges.len() - 1 {
            if ranges[i].to >= ranges[i + 1].from {
                ranges[i + 1].from = cmp::min(ranges[i].from, ranges[i + 1].from);
                ranges[i + 1].to = cmp::max(ranges[i].to, ranges[i + 1].to);
                ranges.remove(i);
                continue 'again;
            }
        }
        break;
    }

    println!(
        "Part one: {}",
        std::fs::read_to_string(path)?
            .lines()
            .filter(|l| !l.contains('-') && !l.is_empty())
            .filter(|value| {
                let value = value.parse::<u64>().unwrap();
                ranges
                    .iter()
                    .any(|range| value >= range.from && value <= range.to)
            })
            .count()
    );
    println!(
        "Part two: {}",
        ranges
            .iter()
            .fold(0, |acc, ranges| acc + (ranges.to + 1) - ranges.from)
    );

    Ok(())
}
