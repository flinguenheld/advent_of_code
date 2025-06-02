#[derive(Debug)]
enum Order {
    CopyVal(i32, usize),
    CopyReg(usize, usize),
    Inc(usize),
    Dec(usize),
    Jump(usize, i32),
}

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let input = std::fs::read_to_string(path).unwrap();
    let orders: Vec<Order> = input
        .lines()
        .map(|l| {
            let mut it = l.split_whitespace();
            let op = it.next().unwrap();
            let x = it.next().unwrap();
            let y = it.next();

            match op {
                "cpy" => {
                    if let Ok(value) = x.parse::<i32>() {
                        Order::CopyVal(value, into_index(y.unwrap()))
                    } else {
                        Order::CopyReg(into_index(x), into_index(y.unwrap()))
                    }
                }
                "inc" => Order::Inc(into_index(x)),
                "dec" => Order::Dec(into_index(x)),
                _ => Order::Jump(into_index(x), y.unwrap().parse::<i32>().unwrap()),
            }
        })
        .collect();

    println!("Part one: {}", proceed(&orders, [0; 4]));
    println!("Part two: {}", proceed(&orders, [0, 0, 1, 0]));
}
fn proceed(orders: &[Order], mut registers: [i32; 4]) -> i32 {
    let mut order_index = 0;
    while let Some(order) = orders.get(order_index) {
        match *order {
            Order::CopyVal(val, reg) => registers[reg] = val,
            Order::CopyReg(from, to) => registers[to] = registers[from],
            Order::Inc(reg) => registers[reg] += 1,
            Order::Dec(reg) => registers[reg] -= 1,
            Order::Jump(reg, jump) => {
                if registers[reg] != 0 {
                    let i = order_index as i32;
                    order_index = i.saturating_add(jump) as usize;
                    continue;
                }
            }
        }
        order_index += 1;
    }

    registers[0]
}

fn into_index(x: &str) -> usize {
    match x {
        "a" => 0,
        "b" => 1,
        "c" => 2,
        _ => 3,
    }
}
