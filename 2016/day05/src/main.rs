use chksum_md5 as md5;
use std::io::Write;

struct Password {
    values: [char; 8],
}
impl Password {
    fn new() -> Password {
        Password { values: [' '; 8] }
    }

    fn set_value(&mut self, value: char, position: usize) {
        if let Some(v) = self.values.get_mut(position) {
            if *v == ' ' {
                *v = value;
            }
        }
    }

    fn is_done(&self) -> bool {
        self.values.iter().all(|v| *v != ' ')
    }

    fn print(&self, title: &str) {
        print!("{}", title);
        for c in self.values {
            match c {
                ' ' => print!("_"),
                _ => print!("{}", c),
            }
        }
        print!("\x1b[{}D", title.len() + 8);
        std::io::stdout().flush().unwrap();
    }
}

fn main() {
    let door_id = "abbhdwsy";
    // let door_id = "abc";
    // let door_id = "abc3231929";

    let mut code = Password::new();
    code.print("Part one: ");

    let mut i = 0;
    let mut position = 0;
    while !code.is_done() {
        loop {
            i += 1;

            if let Ok(digest) = md5::chksum(format!("{}{}", door_id, i)) {
                let db = digest.as_bytes();

                if db[0] == 0 && db[1] == 0 && db[2] <= 0x0f {
                    code.set_value(digest.to_hex_lowercase().chars().nth(5).unwrap(), position);
                    code.print("Part one: ");
                    position += 1;
                    break;
                }
            }
        }
    }
    println!();

    // --
    let mut code = Password::new();
    code.print("Part two: ");

    let mut i = 0;
    while !code.is_done() {
        loop {
            i += 1;

            if let Ok(digest) = md5::chksum(format!("{}{}", door_id, i)) {
                let db = digest.as_bytes();

                if db[0] == 0 && db[1] == 0 && db[2] <= 0x0f {
                    let digest = digest.to_hex_lowercase();
                    code.set_value(
                        digest.chars().nth(6).unwrap(),
                        digest.chars().nth(5).unwrap() as usize - 48,
                    );
                    code.print("Part two: ");
                    break;
                }
            }
        }
    }
    println!();
}
