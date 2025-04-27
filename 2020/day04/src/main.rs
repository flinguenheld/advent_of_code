use constants::*;
use regex::Regex;
use std::fs;

mod constants;

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";
    // let path = "input_example_ok.txt";
    // let path = "input_example_nok.txt";

    let fields = [
        ("byr:", BYR),
        ("iyr:", IYR),
        ("eyr:", EYR),
        ("hgt:", HGT),
        ("hcl:", HCL),
        ("ecl:", ECL),
        ("pid:", PID),
    ];

    let mut part_one = 0;
    let mut part_two = 0;
    for passport in fs::read_to_string(path).unwrap().split("\n\n") {
        if fields.iter().all(|(field, _)| passport.contains(field)) {
            part_one += 1;
        }

        if fields
            .iter()
            .all(|(field, regex)| is_valid(field, regex, passport))
        {
            part_two += 1;
        }
    }

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);
}

fn is_valid(field: &str, regex: &str, passport: &str) -> bool {
    if let Some(data) = passport.split_whitespace().find(|f| f.starts_with(field)) {
        if let Some((_, value)) = data.split_once(':') {
            if let Ok(r) = Regex::new(regex) {
                return r.is_match(value);
            }
        }
    }

    false
}
