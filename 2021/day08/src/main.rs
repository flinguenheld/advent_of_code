use std::collections::{HashMap, HashSet};

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";
    // let path = "input_example_2.txt";

    let mut count = 0;
    for line in std::fs::read_to_string(path).unwrap().lines() {
        let (_, right) = line.split_once(" | ").unwrap();

        count += right
            .split_whitespace()
            .filter(|txt| [2, 4, 3, 7].contains(&txt.chars().count()))
            .count()
    }

    println!("Part one: {}", count);

    // --
    let regular_digits: HashMap<&str, u32> = HashMap::from([
        ("abcefg", 0),
        ("cf", 1),
        ("acdeg", 2),
        ("acdfg", 3),
        ("bcdf", 4),
        ("abdfg", 5),
        ("abdefg", 6),
        ("acf", 7),
        ("abcdefg", 8),
        ("abcdfg", 9),
    ]);

    let mut count = 0;
    for line in std::fs::read_to_string(path).unwrap().lines() {
        let mut who: HashMap<char, char> = HashMap::new();

        let (left, right) = line.split_once(" | ").unwrap();
        let left: Vec<HashSet<char>> = left
            .split_whitespace()
            .map(|txt| txt.chars().collect())
            .collect();

        let one = left.iter().find(|l| l.len() == 2).unwrap();
        let four = left.iter().find(|l| l.len() == 4).unwrap();
        let seven = left.iter().find(|l| l.len() == 3).unwrap();
        let eight = left.iter().find(|l| l.len() == 7).unwrap();

        let two_three_five: Vec<HashSet<char>> =
            left.iter().filter(|l| l.len() == 5).cloned().collect();

        let three = two_three_five
            .iter()
            .find(|l| one.iter().all(|v| l.contains(v)))
            .unwrap();

        who.insert('a', *one.symmetric_difference(seven).next().unwrap());
        who.insert(
            'g',
            *three
                .iter()
                .find(|v| !seven.contains(v) && !four.contains(v))
                .unwrap(),
        );
        who.insert(
            'd',
            *three
                .iter()
                .find(|v| !one.contains(v) && !who.values().any(|w| w == *v))
                .unwrap(),
        );
        who.insert(
            'b',
            *four
                .iter()
                .find(|v| !one.contains(v) && !who.values().any(|w| w == *v))
                .unwrap(),
        );

        let five = two_three_five
            .iter()
            .find(|l| l.contains(who.get(&'b').unwrap()))
            .unwrap();

        who.insert(
            'f',
            *five
                .iter()
                .find(|v| !who.values().any(|w| w == *v))
                .unwrap(),
        );
        who.insert(
            'c',
            *four
                .iter()
                .find(|v| !who.values().any(|w| w == *v))
                .unwrap(),
        );
        who.insert(
            'e',
            *eight
                .iter()
                .find(|v| !who.values().any(|w| w == *v))
                .unwrap(),
        );

        // Reverse who to easily decode
        let who: HashMap<char, char> = who.iter().map(|(k, v)| (*v, *k)).collect();

        let mut decades = 1;
        let mut current_digit = 0;
        for to_decode in right.split_whitespace().rev() {
            let decoded: Vec<char> = to_decode
                .chars()
                .map(|c| who.get(&c).unwrap())
                .cloned()
                .collect();

            if let Some((_, v)) = regular_digits.iter().find(|(key, _)| {
                key.chars().count() == decoded.len() && decoded.iter().all(|c| key.contains(*c))
            }) {
                current_digit += v * decades;
                decades *= 10;
            }
        }

        count += current_digit;
    }

    println!("Part two: {}", count);
}
