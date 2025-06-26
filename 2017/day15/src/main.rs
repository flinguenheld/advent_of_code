fn main() {
    // let (mut base_a, mut base_b) = (65_u64, 8921_u64);
    let (base_a, base_b) = (883_u64, 879_u64);

    let mask_16 = 0b1111111111111111;

    let mut a = base_a;
    let mut b = base_b;

    let mut total = 0;
    for _ in 0..40_000_000 {
        a = a * 16807 % 2147483647;
        b = b * 48271 % 2147483647;

        if a & mask_16 == b & mask_16 {
            total += 1;
        }
    }
    println!("Part one: {}", total);

    // --
    let mut a = base_a;
    let mut b = base_b;

    let mut total = 0;
    for _ in 0..5_000_000 {
        a = next(a, 16807, 4);
        b = next(b, 48271, 8);

        if a & mask_16 == b & mask_16 {
            total += 1;
        }
    }
    println!("Part two: {}", total);
}

fn next(mut num: u64, mul: u64, modulo: u64) -> u64 {
    loop {
        num = num * mul % 2147483647;

        if num.rem_euclid(modulo) == 0 {
            break;
        }
    }
    num
}
