use std::collections::VecDeque;

fn main() {
    println!("Part one: {}", get_ten_next(702831));
    println!("Part two: {}", generate_until(vec![7, 0, 2, 8, 3, 1]));
}

fn get_ten_next(input: usize) -> String {
    let mut recipes = vec![3, 7];

    let mut elf_1 = 0;
    let mut elf_2 = 1;

    // for _ in 0..input {
    while recipes.len() < input + 10 {
        let a = recipes[elf_1];
        let b = recipes[elf_2];

        recipes.extend(get_digits(a + b).iter());

        elf_1 = (elf_1 + 1 + recipes[elf_1]) % recipes.len();
        elf_2 = (elf_2 + 1 + recipes[elf_2]) % recipes.len();
    }

    recipes
        .iter()
        .skip(input)
        .take(10)
        .map(|c| c.to_string())
        .collect::<String>()
}

fn generate_until(until: Vec<usize>) -> usize {
    let mut recipes = vec![3, 7];

    let mut elf_1 = 0;
    let mut elf_2 = 1;

    while !recipes.ends_with(&until) {
        let a = recipes[elf_1];
        let b = recipes[elf_2];

        recipes.extend(get_digits(a + b).iter());

        elf_1 = (elf_1 + 1 + recipes[elf_1]) % recipes.len();
        elf_2 = (elf_2 + 1 + recipes[elf_2]) % recipes.len();
    }

    recipes
        .windows(until.len())
        .position(|win| win == until)
        .unwrap()
}

fn get_digits(mut new: usize) -> VecDeque<usize> {
    let mut to_add = VecDeque::new();
    if new == 0 {
        to_add.push_front(0);
    } else {
        while new > 0 {
            to_add.push_front(new % 10);
            new /= 10;
        }
    }
    to_add
}

// --
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn get_ten() {
        assert_eq!(get_ten_next(9), "5158916779".to_string());
        assert_eq!(get_ten_next(5), "0124515891".to_string());
        assert_eq!(get_ten_next(18), "9251071085".to_string());
        assert_eq!(get_ten_next(2018), "5941429882".to_string());
    }

    #[test]
    fn get_until() {
        assert_eq!(generate_until(vec![5, 1, 5, 8, 9]), 9);
        assert_eq!(generate_until(vec![0, 1, 2, 4, 5]), 5);
        assert_eq!(generate_until(vec![9, 2, 5, 1, 0]), 18);
        assert_eq!(generate_until(vec![5, 9, 4, 1, 4]), 2018);
    }

    #[test]
    fn digits() {
        assert_eq!(get_digits(2018), vec![2, 0, 1, 8]);
        assert_eq!(get_digits(5050), vec![5, 0, 5, 0]);
        assert_eq!(get_digits(1000), vec![1, 0, 0, 0]);
        assert_eq!(get_digits(1), vec![1]);
        assert_eq!(get_digits(0), vec![0]);
    }
}
