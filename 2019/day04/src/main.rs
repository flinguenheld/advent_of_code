use std::collections::HashSet;

fn main() {
    let mut count = 0;
    let mut count_2 = 0;
    for i in 172851..=675869 {
        if is_ok(i) {
            count += 1;
        }
        if is_ok_2(i) {
            count_2 += 1;
        }
    }
    println!("Part one: {}", count);
    println!("Part one: {}", count_2);
}

fn is_ok(mut number: u32) -> bool {
    let mut n = Vec::new();
    for _ in 0..6 {
        n.push(number % 10);
        number /= 10;
    }

    let mut same_ok = false;
    for win in n.windows(2).rev() {
        if win[0] == win[1] {
            same_ok = true;
        }

        if win[0] < win[1] {
            return false;
        }
    }

    same_ok
}

fn is_ok_2(mut number: u32) -> bool {
    let mut n = Vec::new();

    for _ in 0..6 {
        n.push(number % 10);
        number /= 10;
    }

    let mut ok: HashSet<u32> = HashSet::new();
    let mut nok: HashSet<u32> = HashSet::new();

    for win in n.windows(3).rev() {
        if win[2] == win[1] || win[1] == win[0] {
            ok.insert(win[1]);
        }
        if win[2] == win[1] && win[1] == win[0] {
            nok.insert(win[1]);
        }

        if win[0] < win[1] || win[1] < win[2] {
            return false;
        }
    }

    ok.difference(&nok).count() > 0
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn is_ok_really_ok() {
        assert!(is_ok(111111));
        assert!(is_ok(123455));
    }

    #[test]
    fn is_ok_really_nok() {
        assert!(!is_ok(123456));
        assert!(!is_ok(123451));
        assert!(!is_ok(211111));
        assert!(!is_ok(223450));
        assert!(!is_ok(123789));
    }

    #[test]
    fn is_ok_2_really_ok() {
        assert!(is_ok_2(112233));
        assert!(is_ok_2(111122));
    }

    #[test]
    fn is_ok_2_really_nok() {
        assert!(!is_ok_2(123444));
        assert!(!is_ok_2(123334));
        assert!(!is_ok_2(444529));
        assert!(!is_ok_2(646688));
    }
}
