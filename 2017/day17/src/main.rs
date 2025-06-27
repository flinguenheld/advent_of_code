fn main() {
    // let input = 3;
    let input = 366;

    let mut spinlock = vec![0, 1];
    let mut index = 1;

    let mut new_length = 2;
    loop {
        index = (index + input) % new_length;
        spinlock.insert(index, new_length);

        index += 1;
        new_length += 1;

        if new_length > 2017 {
            println!("Part one: {}", spinlock[index]);
            break;
        }
    }

    // --
    let mut spinlock = [0, 1];
    let mut index = 1;

    for length in 2..=50_000_000 {
        index = (index + input) % (length);
        if index == 0 {
            spinlock[1] = length;
        }

        index += 1;
    }

    println!("Part two: {}", spinlock[1]);
}
