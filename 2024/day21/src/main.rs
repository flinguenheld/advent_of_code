use std::{
    collections::{HashMap, VecDeque},
    fs, mem,
};

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let input = fs::read_to_string(path).unwrap();
    let mut input: Vec<VecDeque<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let keypad = HashMap::from([
        ('7', (0, 0)),
        ('8', (0, 1)),
        ('9', (0, 2)),
        ('4', (1, 0)),
        ('5', (1, 1)),
        ('6', (1, 2)),
        ('1', (2, 0)),
        ('2', (2, 1)),
        ('3', (2, 2)),
        ('0', (3, 1)),
        ('A', (3, 2)),
    ]);

    let keypad_dir = HashMap::from([
        ('^', (0, 1)),
        ('A', (0, 2)),
        ('<', (1, 0)),
        ('v', (1, 1)),
        ('>', (1, 2)),
    ]);

    // dbg!(&input);
    let mut total = 0;
    for command in input.iter_mut() {
        let mut conversions_from: Vec<VecDeque<char>> = Vec::new();
        let mut conversions_to: Vec<VecDeque<char>> = Vec::new();

        // Step 1 --
        command.push_front('A');
        next(
            &keypad,
            command.clone(),
            VecDeque::new(),
            &mut conversions_from,
        );

        // --
        for _ in 0..2 {
            for conv in conversions_from.iter_mut() {
                conv.push_front('A');
                next(
                    &keypad_dir,
                    conv.clone(),
                    VecDeque::new(),
                    &mut conversions_to,
                );
            }

            mem::swap(&mut conversions_from, &mut conversions_to);
            conversions_to.clear();
        }

        conversions_from.sort_by_key(|c| c.len());
        println!("mini: {}", conversions_from.first().unwrap().len());
        println!("maxi: {}", conversions_from.last().unwrap().len());
        if let Some(first) = conversions_from.first() {
            let value = command
                .iter()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>();
            total += first.len() * value.parse::<usize>().unwrap();

            println!("Command: {:?}  ->  {} * {}", command, first.len(), value);
        }
    }

    println!("Part one: {}", total);
}

fn next(
    keypad: &HashMap<char, (i32, i32)>,
    mut commands: VecDeque<char>,
    mut current_conversion: VecDeque<char>,
    conversions: &mut Vec<VecDeque<char>>,
) {
    if let Some(from) = commands.pop_front() {
        if let Some(to) = commands.front() {
            let vertical = keypad.get(to).unwrap().0 - keypad.get(&from).unwrap().0;
            let horizontal = keypad.get(to).unwrap().1 - keypad.get(&from).unwrap().1;

            if vertical == 0 && horizontal != 0 {
                hop_horizontal(horizontal, &mut current_conversion);
                current_conversion.push_back('A');
                next(keypad, commands, current_conversion, conversions);
            } else if vertical != 0 && horizontal == 0 {
                hop_vertical(vertical, &mut current_conversion);
                current_conversion.push_back('A');
                next(keypad, commands, current_conversion, conversions);
            } else if vertical != 0 && horizontal != 0 {
                if !((*to == '0' || *to == 'A') && (from == '7' || from == '4' || from == '1')
                    || ((*to == '^' || *to == 'A') && from == '<'))
                {
                    let mut cur = current_conversion.clone();
                    hop_vertical(vertical, &mut cur);
                    hop_horizontal(horizontal, &mut cur);
                    cur.push_back('A');
                    next(keypad, commands.clone(), cur, conversions);
                }
                if !((*to == '1' || *to == '4' || *to == '7') && (from == '0' || from == 'A')
                    || ((from == '^' || from == 'A') && *to == '<'))
                {
                    hop_horizontal(horizontal, &mut current_conversion);
                    hop_vertical(vertical, &mut current_conversion);
                    current_conversion.push_back('A');
                    next(keypad, commands, current_conversion, conversions);
                }
            } else {
                current_conversion.push_back('A');
                next(keypad, commands.clone(), current_conversion, conversions);
            }
        } else {
            conversions.push(current_conversion);
        }
    }
}

#[rustfmt::skip]
fn hop_horizontal(distance: i32, convertion: &mut VecDeque<char>) {
    match distance.cmp(&0) {
        std::cmp::Ordering::Greater => { (0..distance).for_each(|_| convertion.push_back('>')); }
        std::cmp::Ordering::Less => { (distance..0).for_each(|_| convertion.push_back('<')); }
        _ => {}
    }
}

#[rustfmt::skip]
fn hop_vertical(distance: i32, convertion: &mut VecDeque<char>) {
    match distance.cmp(&0) {
        std::cmp::Ordering::Greater => { (0..distance).for_each(|_| convertion.push_back('v')); }
        std::cmp::Ordering::Less => { (distance..0).for_each(|_| convertion.push_back('^')); }
        _ => {}
    }
}
