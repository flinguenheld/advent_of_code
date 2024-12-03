use regex::Regex;
use std::fs;

fn main() {
    // let path = "input_example.txt";
    // let path = "input_example_2.txt";
    let path = "input.txt";
    let input = fs::read_to_string(path).unwrap();

    // Part one --
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let matches: Vec<_> = re.find_iter(&input).map(|m| m.as_str()).collect();

    println!("Part one: {}", count(&matches));

    // Part two --
    let re = Regex::new(r"(mul\(\d{1,3},\d{1,3}\))|(do\(\))|(don't\(\))").unwrap();
    let matches: Vec<_> = re.find_iter(&input).map(|m| m.as_str()).collect();

    let mut filtered = Vec::new();
    let mut include = true;
    for command in matches.iter() {
        match *command {
            "do()" => {
                include = true;
            }
            "don't()" => {
                include = false;
            }
            _ => {
                if include {
                    filtered.push(*command)
                }
            }
        }
    }

    println!("Part two: {}", count(&filtered));
}

fn count(commands: &[&str]) -> u32 {
    commands.iter().fold(0, |acc, calcul| {
        let (l, r) = calcul.split_once(',').unwrap();
        let l = parse(l);
        let r = parse(r);
        acc + r * l
    })
}
fn parse(val: &str) -> u32 {
    let val: String = val.chars().filter(|c| c.is_ascii_digit()).collect();
    val.parse::<u32>().unwrap()
}
