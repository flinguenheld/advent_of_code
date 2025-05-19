#[derive(Debug)]
struct Ingredient {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

fn to_num(input: &str) -> i64 {
    input
        .chars()
        .filter(|c| c.is_ascii_digit() || *c == '-')
        .collect::<String>()
        .parse::<i64>()
        .unwrap()
}

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let input = std::fs::read_to_string(path).unwrap();
    let ingredients: Vec<Ingredient> = input
        .lines()
        .map(|line| {
            let mut it = line.split_whitespace();
            Ingredient {
                capacity: to_num(it.nth(2).unwrap()),
                durability: to_num(it.nth(1).unwrap()),
                flavor: to_num(it.nth(1).unwrap()),
                texture: to_num(it.nth(1).unwrap()),
                calories: to_num(it.last().unwrap()),
            }
        })
        .collect();

    println!("Part one: {}", get_max(Vec::new(), 100, &ingredients, 0));
    println!("Part two: {}", get_max(Vec::new(), 100, &ingredients, 500));
}

fn get_max(
    mut combinations: Vec<i64>,
    total: i64,
    ingredients: &Vec<Ingredient>,
    calories_to_reach: i64,
) -> i64 {
    if combinations.len() < ingredients.len() - 1 {
        let mut max = 0;
        for i in 1..(total - combinations.iter().sum::<i64>()) {
            let mut new_combination = combinations.clone();
            new_combination.push(i);
            max = std::cmp::max(
                max,
                get_max(new_combination, total, ingredients, calories_to_reach),
            );
        }
        max
    } else {
        combinations.push(total - combinations.iter().sum::<i64>());

        let (mut capa, mut dura, mut flav, mut text, mut cal) = (0, 0, 0, 0, 0);
        for (comb, ing) in combinations.iter().zip(ingredients.iter()) {
            capa += comb * ing.capacity;
            dura += comb * ing.durability;
            flav += comb * ing.flavor;
            text += comb * ing.texture;
            if calories_to_reach > 0 {
                cal += comb * ing.calories;
            }
        }

        if cal == calories_to_reach && capa > 0 && dura > 0 && flav > 0 && text > 0 {
            capa * dura * flav * text
        } else {
            0
        }
    }
}
