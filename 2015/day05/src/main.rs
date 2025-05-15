const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
const FORBID: [[char; 2]; 4] = [['a', 'b'], ['c', 'd'], ['p', 'q'], ['x', 'y']];

fn main() {
    // let path = "input_example.txt";
    // let path = "input_example_2.txt";
    let path = "input.txt";

    let mut input: Vec<Vec<char>> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    println!(
        "Part one: {}",
        input.iter().fold(0, |acc, row| {
            match row.iter().filter(|c| VOWELS.contains(c)).count() >= 3
                && row.windows(2).any(|win| win[0] == win[1])
                && !row.windows(2).any(|win| FORBID.contains(&[win[0], win[1]]))
            {
                true => acc + 1,
                false => acc,
            }
        })
    );

    // --
    input.retain(|row| {
        for (i, win) in row.windows(2).enumerate() {
            for win2 in row.windows(2).skip(i + 2) {
                if win == win2 {
                    for win3 in row.windows(3) {
                        if win3[0] == win3[2] {
                            return true;
                        }
                    }
                }
            }
        }
        false
    });
    println!("Part two: {}", input.len());
}
