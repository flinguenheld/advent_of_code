fn main() {
    // let mut input = "abcdef";
    // let mut input = "pqrstuv";
    let input = "ckczppom";

    let mut i = 1;
    loop {
        i += 1;
        let test = md5::compute(format!("{}{}", input, i));
        if test[0] == 0 && test[1] == 0 && test[2] < 17 {
            println!("Part one: {}", i);
            break;
        }
    }

    // --
    let mut i = 1;
    loop {
        i += 1;
        let test = md5::compute(format!("{}{}", input, i));
        if test[..3].iter().all(|v| *v == 0) {
            println!("Part two: {}", i);
            break;
        }
    }
}
