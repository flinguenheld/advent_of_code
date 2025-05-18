use std::fmt;

struct Character(u8);

impl Character {
    fn new(value: char) -> Self {
        Character(value as u8 - 97)
    }
    fn to_char(&self) -> char {
        (self.0 + 97) as char
    }

    fn is_forbiden(&self) -> bool {
        [8, 11, 14].contains(&self.0)
    }

    fn next(&mut self) -> bool {
        self.0 += 1;

        if self.is_forbiden() {
            self.0 += 1;
        }

        if self.0 > 25 {
            self.0 = 0;
            true
        } else {
            false
        }
    }
}

struct Password(Vec<Character>);

impl Password {
    fn new(value: &str) -> Self {
        Password(
            value
                .chars()
                .map(Character::new)
                .collect::<Vec<Character>>(),
        )
    }
    fn next(&mut self) {
        let mut again = true;
        for c in self.0.iter_mut().rev() {
            if again {
                again = c.next();
            }
        }
    }

    fn is_valid(&self) -> bool {
        if self.0.iter().all(|v| !v.is_forbiden()) {
            for win in self.0.windows(3) {
                if win[2].0 == win[1].0 + 1 && win[1].0 == win[0].0 + 1 {
                    for (i, w) in self.0.windows(2).enumerate() {
                        if w[0].0 == w[1].0 {
                            for w2 in self.0.windows(2).skip(i + 2) {
                                if w2[0].0 == w2[1].0 {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }
        false
    }

    fn next_pass(mut self) -> Self {
        loop {
            self.next();
            if self.is_valid() {
                break;
            }
        }
        self
    }
}
impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0.iter().map(|v| v.to_char()).collect::<String>()
        )
    }
}

fn main() {
    // let input = "abcdefgh";
    // let input = "ghijklmn";
    let input = "hepxcrrq";

    let pass = Password::new(input);

    let pass = pass.next_pass();
    println!("Part one: {}", pass);

    let pass = pass.next_pass();
    println!("Part two: {}", pass);
}
