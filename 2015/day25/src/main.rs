const MUL: u64 = 252533;
const DIV: u64 = 33554393;

fn main() {
    let input = (2978 - 1, 3083 - 1);
    // let input = (5, 5);

    let mut table: Vec<Vec<u64>> = vec![vec![20151125]];
    let mut current_value = 20151125;

    loop {
        table.push(Vec::new());

        for r in table.iter_mut().rev() {
            current_value = (current_value * MUL).rem_euclid(DIV);
            r.push(current_value);
        }

        if let Some(row) = table.get(input.0) {
            if let Some(col) = row.get(input.1) {
                println!("Part one: {}", col);
                break;
            }
        }
    }
}
