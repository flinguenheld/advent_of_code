use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug)]
struct Order<'a> {
    low_to: Rc<RefCell<HashMap<&'a str, Vec<u32>>>>,
    low_id: &'a str,
    high_to: Rc<RefCell<HashMap<&'a str, Vec<u32>>>>,
    high_id: &'a str,
}

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let mut orders: HashMap<&str, Order> = HashMap::new();

    let bots: Rc<RefCell<HashMap<&str, Vec<u32>>>> = Rc::new(RefCell::new(HashMap::new()));
    let outputs: Rc<RefCell<HashMap<&str, Vec<u32>>>> = Rc::new(RefCell::new(HashMap::new()));

    let input = std::fs::read_to_string(path).unwrap();

    for line in input.lines().filter(|l| l.starts_with("value")) {
        let mut it = line.split_whitespace();

        let value = it.nth(1).unwrap().parse::<u32>().unwrap();
        let bot_id = it.last().unwrap();

        bots.borrow_mut()
            .entry(bot_id)
            .and_modify(|b| b.push(value))
            .or_insert(vec![value]);
    }

    // Record all orders
    for line in input.lines().filter(|l| l.starts_with("bot")) {
        let mut it = line.split_whitespace();

        let bot_id = it.nth(1).unwrap();
        let low_to = it.nth(3).unwrap();
        let low_id = it.next().unwrap();
        let high_to = it.nth(3).unwrap();
        let high_id = it.last().unwrap();

        orders.insert(
            bot_id,
            Order {
                low_to: match low_to {
                    "bot" => bots.clone(),
                    _ => outputs.clone(),
                },
                low_id,
                high_to: match high_to {
                    "bot" => bots.clone(),
                    _ => outputs.clone(),
                },
                high_id,
            },
        );
    }

    'blah: loop {
        // Get names and then proceed to respect the RefCell rules
        let bots_with_two_chips: Vec<&str> = bots
            .borrow()
            .iter()
            .filter(|(_, v)| v.len() > 1)
            .map(|(name, _)| *name)
            .collect();

        for name in bots_with_two_chips.iter() {
            let (high, low) = {
                let mut bots = bots.borrow_mut();
                let values = bots.get_mut(name).unwrap();
                values.sort();
                (values.pop().unwrap(), values.pop().unwrap())
            };

            let order = orders.get(name).unwrap();

            order
                .low_to
                .borrow_mut()
                .entry(order.low_id)
                .and_modify(|v| v.push(low))
                .or_insert(vec![low]);
            order
                .high_to
                .borrow_mut()
                .entry(order.high_id)
                .and_modify(|v| v.push(high))
                .or_insert(vec![high]);

            if low == 17 && high == 61 {
                println!("Part one: {}", name);
            }

            if let Some(product) = (0..3).try_fold(1, |acc, i| {
                outputs
                    .borrow()
                    .get(i.to_string().as_str())
                    .map(|v| acc * v.last().unwrap())
            }) {
                println!("Part two: {}", product);
                break 'blah;
            }
        }
    }
}
