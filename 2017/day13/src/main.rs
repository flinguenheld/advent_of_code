#[derive(Debug, Clone)]
struct Scanner {
    up: bool,
    length: usize,
    current: usize,
}

impl Scanner {
    fn new(length: usize) -> Self {
        Scanner {
            up: true,
            length,
            current: 0,
        }
    }

    fn next(&mut self) {
        if self.length > 0 {
            match self.up {
                true => self.current += 1,
                false => self.current -= 1,
            };

            if self.current == 0 {
                self.up = true;
            } else if self.current == self.length - 1 {
                self.up = false;
            }
        }
    }
}

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let mut input: Vec<Scanner> = Vec::new();
    let mut index = 0;
    for line in std::fs::read_to_string(path).unwrap().lines() {
        let (left, right) = line.split_once(": ").unwrap();

        let left = left.parse::<usize>().unwrap();
        let right = right.parse::<usize>().unwrap();

        while left > index {
            input.push(Scanner::new(0));
            index += 1;
        }

        input.push(Scanner::new(right));
        index += 1;
    }
    // dbg!(&input);

    let mut scanners = input.clone();

    let mut total = 0;
    for i in 0..scanners.len() {
        if scanners[i].length > 0 && scanners[i].current == 0 {
            total += i * scanners[i].length;
        }

        scanners.iter_mut().for_each(|s| s.next());
    }

    println!("Part one: {}", total);

    // --
    let mut i = 0;
    let mut scanners = input.clone();

    while !test(scanners.clone()) {
        i += 1;
        scanners.iter_mut().for_each(|s| s.next());
    }
    println!("Part two: {}", i);
}

fn test(mut scanners: Vec<Scanner>) -> bool {
    for i in 0..scanners.len() {
        if scanners[i].length > 0 && scanners[i].current == 0 {
            return false;
        }

        scanners.iter_mut().for_each(|s| s.next());
    }
    true
}
