use chksum_md5 as md5;

#[derive(Debug)]
struct Hash {
    index: u32,
    sequence: char,
}

fn main() {
    // let input = "abc";
    let input = "qzyelonm";

    println!("Part one: {}", get_sixty_fourth_index(input, false));
    println!("Part two: {}", get_sixty_fourth_index(input, true));
}

fn get_sixty_fourth_index(input: &str, stretch: bool) -> u32 {
    let mut index = 0;

    let mut three: Vec<Hash> = Vec::new();
    let mut valids: Vec<u32> = Vec::new();

    loop {
        let tab = if stretch {
            gimme_the_hash(format!("{}{}", input, index))
        } else {
            md5::chksum(format!("{}{}", input, index))
                .unwrap()
                .to_hex_lowercase()
        }
        .chars()
        .collect::<Vec<char>>();

        // Contains a five in a row ?
        for win in tab.windows(5) {
            if win.iter().skip(1).all(|c| *c == win[0]) {
                while let Some(pos) = three.iter().position(|three| {
                    three.index as i32 >= (index as i32 - 1000)
                        && three.index <= index
                        && three.sequence == win[0]
                }) {
                    let aa = three.remove(pos).index;
                    valids.push(aa);
                }
            }
        }

        // Here because valid values are not found in the order !!!
        if valids.len() >= 64 {
            valids.sort();
            return *valids.get(63).unwrap();
        }

        // Contains a three in a row ?
        for win in tab.windows(3) {
            if win.iter().skip(1).all(|c| *c == win[0]) {
                three.push(Hash {
                    index,
                    sequence: win[0],
                });
                break;
            }
        }
        index += 1;
    }
}

fn gimme_the_hash(mut value: String) -> String {
    for _ in 0..=2016 {
        value = md5::chksum(value).unwrap().to_hex_lowercase();
    }
    value
}
