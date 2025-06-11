fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    println!("Part one: {}", scramble("abcdefgh", path));
    println!("Part two: {}", unscramble("fbgdceah", path));
}

fn scramble(text: &str, path: &str) -> String {
    let mut text = text.chars().collect::<Vec<char>>();

    for line in std::fs::read_to_string(path).unwrap().lines() {
        let fields: Vec<&str> = line.split_ascii_whitespace().collect();

        match fields[0] {
            "swap" => {
                if fields[1] == "position" {
                    text.swap(
                        fields[2].parse::<usize>().unwrap(),
                        fields[5].parse::<usize>().unwrap(),
                    );
                } else {
                    let left = text
                        .iter()
                        .position(|c| *c == fields[2].chars().last().unwrap())
                        .unwrap();
                    let right = text
                        .iter()
                        .position(|c| *c == fields[5].chars().last().unwrap())
                        .unwrap();

                    text.swap(left, right);
                }
            }
            "rotate" => {
                if fields[1] == "based" {
                    let letter = fields[6].chars().last().unwrap();
                    text = rotate(text, letter);
                } else {
                    let times = fields[2].parse::<usize>().unwrap();

                    if fields[1] == "right" {
                        text.rotate_right(times);
                    } else {
                        text.rotate_left(times);
                    }
                }
            }
            "reverse" => {
                let from_included = fields[2].parse::<usize>().unwrap();
                let to_included = fields[4].parse::<usize>().unwrap();

                let start: Vec<char> = text[..from_included].to_vec();
                let middle: Vec<char> = text[from_included..=to_included]
                    .iter()
                    .rev()
                    .cloned()
                    .collect();
                let end: Vec<char> = text[to_included + 1..].to_vec();

                text.clear();
                text.extend(start);
                text.extend(middle);
                text.extend(end);
            }
            _ => {
                let from = fields[2].parse::<usize>().unwrap();
                let to = fields[5].parse::<usize>().unwrap();

                let letter = text.remove(from);
                text.insert(to, letter);
            }
        }
    }
    text.iter().collect::<String>()
}

fn unscramble(text: &str, path: &str) -> String {
    let mut text = text.chars().collect::<Vec<char>>();

    for line in std::fs::read_to_string(path).unwrap().lines().rev() {
        let fields: Vec<&str> = line.split_ascii_whitespace().collect();

        match fields[0] {
            "swap" => {
                if fields[1] == "position" {
                    text.swap(
                        fields[2].parse::<usize>().unwrap(),
                        fields[5].parse::<usize>().unwrap(),
                    );
                } else {
                    let left = text
                        .iter()
                        .position(|c| *c == fields[2].chars().last().unwrap())
                        .unwrap();
                    let right = text
                        .iter()
                        .position(|c| *c == fields[5].chars().last().unwrap())
                        .unwrap();

                    text.swap(left, right);
                }
            }
            "rotate" => {
                if fields[1] == "based" {
                    let letter = fields[6].chars().last().unwrap();

                    let previous_text = text.clone();
                    while rotate(text.clone(), letter) != previous_text {
                        text.rotate_left(1);
                    }
                } else {
                    let times = fields[2].parse::<usize>().unwrap();

                    if fields[1] == "right" {
                        text.rotate_left(times);
                    } else {
                        text.rotate_right(times);
                    }
                }
            }
            "reverse" => {
                let from_included = fields[2].parse::<usize>().unwrap();
                let to_included = fields[4].parse::<usize>().unwrap();

                let start: Vec<char> = text[..from_included].to_vec();
                let middle: Vec<char> = text[from_included..=to_included]
                    .iter()
                    .rev()
                    .cloned()
                    .collect();
                let end: Vec<char> = text[to_included + 1..].to_vec();

                text.clear();
                text.extend(start);
                text.extend(middle);
                text.extend(end);
            }
            _ => {
                let from = fields[5].parse::<usize>().unwrap();
                let to = fields[2].parse::<usize>().unwrap();

                let letter = text.remove(from);
                text.insert(to, letter);
            }
        }
    }
    text.iter().collect::<String>()
}

fn rotate(mut text: Vec<char>, letter: char) -> Vec<char> {
    let position = text.iter().position(|c| *c == letter).unwrap();

    text.rotate_right(1);
    if position >= 4 {
        text.rotate_right(position);
        text.rotate_right(1);
    } else {
        text.rotate_right(position);
    }

    text
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn abcde() {
        let scrambled = scramble("abcde", "input_example.txt");
        let unscrambled = unscramble(scrambled.as_str(), "input_example.txt");
        assert_eq!("abcde", unscrambled);
    }
    #[test]
    fn decab() {
        let unscrambled = unscramble("decab", "input_example.txt");
        let scrambled = scramble(unscrambled.as_str(), "input_example.txt");
        assert_eq!("decab", scrambled);
    }

    #[test]
    fn abcdefgh() {
        let scrambled = scramble("abcdefgh", "input.txt");
        let unscrambled = unscramble(scrambled.as_str(), "input.txt");
        assert_eq!("abcdefgh", unscrambled);
    }
    #[test]
    fn fbgdceah() {
        let unscrambled = unscramble("fbgdceah", "input.txt");
        let scrambled = scramble(unscrambled.as_str(), "input.txt");
        assert_eq!("fbgdceah", scrambled);
    }
}
