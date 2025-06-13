#[derive(Debug, Clone, Copy)]
enum Type {
    Register(usize),
    Regular(i32),
}

#[derive(Debug, Clone, Copy)]
enum Order {
    Copy(Type, usize),
    Inc(usize),
    Dec(usize),
    Jump(Type, Type),
    Toggle(usize),
}

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let input = std::fs::read_to_string(path).unwrap();
    let orders: Vec<Order> = input
        .lines()
        .map(|l| {
            let fields: Vec<&str> = l.split_whitespace().collect();
            let op = fields[0];
            let x = fields[1];

            match op {
                "cpy" => {
                    let x = if let Ok(value) = x.parse::<i32>() {
                        Type::Regular(value)
                    } else {
                        Type::Register(into_index(x))
                    };

                    Order::Copy(x, into_index(fields[2]))
                }
                "inc" => Order::Inc(into_index(x)),
                "dec" => Order::Dec(into_index(x)),
                "jnz" => {
                    let x = if let Ok(value) = x.parse::<i32>() {
                        Type::Regular(value)
                    } else {
                        Type::Register(into_index(x))
                    };
                    let y = if let Ok(value) = fields[2].parse::<i32>() {
                        Type::Regular(value)
                    } else {
                        Type::Register(into_index(fields[2]))
                    };

                    Order::Jump(x, y)
                }
                _ => Order::Toggle(into_index(x)),
            }
        })
        .collect();

    println!("Part one: {}", proceed(&mut orders.clone(), [7, 0, 0, 0]));
    println!("Part two: {}", proceed(&mut orders.clone(), [12, 0, 0, 0]));
}

fn proceed(orders: &mut [Order], mut registers: [i32; 4]) -> i32 {
    let mut order_index = 0;
    while let Some(order) = orders.get(order_index) {
        match *order {
            Order::Toggle(reg) => {
                if registers[reg] > 0 {
                    if let Some(to_update) = orders.get_mut(order_index + registers[reg] as usize) {
                        *to_update = match to_update {
                            Order::Copy(a, b) => Order::Jump(*a, Type::Register(*b)),
                            Order::Jump(a, b) => {
                                let b = match b {
                                    Type::Register(b) => *b,
                                    _ => 0,
                                };
                                Order::Copy(*a, b)
                            }
                            Order::Inc(reg) => Order::Dec(*reg),
                            Order::Dec(reg) | Order::Toggle(reg) => Order::Inc(*reg),
                        };
                    }
                }
            }
            Order::Copy(from, into) => {
                let value = match from {
                    Type::Register(r) => registers[r],
                    Type::Regular(v) => v,
                };
                registers[into] = value;
            }

            Order::Inc(reg) => registers[reg] += 1,
            Order::Dec(reg) => registers[reg] -= 1,
            Order::Jump(state, steps) => {
                let state = match state {
                    Type::Register(r) => registers[r],
                    Type::Regular(v) => v,
                };
                let steps = match steps {
                    Type::Register(r) => registers[r],
                    Type::Regular(v) => v,
                };

                if state != 0 {
                    let i = order_index as i32;
                    order_index = i.saturating_add(steps) as usize;
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
