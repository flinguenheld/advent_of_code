use itertools::Itertools;

#[derive(Clone, Debug)]
struct Player {
    hit_points: i32,
    damage: i32,
    armor: i32,
}

#[derive(Debug)]
struct Item {
    cost: i32,
    damage: i32,
    armor: i32,
}
impl Item {
    fn new(cost: i32, damage: i32, armor: i32) -> Self {
        Item {
            cost,
            damage,
            armor,
        }
    }
}

#[rustfmt::skip]
fn main() {
    let shop = std::fs::read_to_string("shop.txt").unwrap();
    let mut shop = shop.split("\n\n");

    let weapons = shop .next() .unwrap() .lines() .skip(1) .map(parse_line) .collect::<Vec<Item>>();
    let mut armors = shop .next() .unwrap() .lines() .skip(1) .map(parse_line) .collect::<Vec<Item>>();
    let rings = shop .next() .unwrap() .lines() .skip(1) .map(parse_line) .collect::<Vec<Item>>();

    // Add "without armor" to be easily combined with rings
    armors.push(Item { cost: 0, damage: 0, armor: 0 });

    let mut mini_win = i32::MAX;
    let mut maxi_loose = 0;

    let me = Player {
        hit_points: 100,
        damage: 0,
        armor: 0,
    };
    let boss = Player {
        hit_points: 103,
        damage: 9,
        armor: 2,
    };

    // For each weapon --
    for weapon in weapons.iter() {
        let mut me = me.clone();
        let mut cost = 0;

        cost += weapon.cost;
        me.damage += weapon.damage;

        // with each armor --
        for armor in armors.iter() {
            let mut me = me.clone();
            let mut cost = cost;

            cost += armor.cost;
            me.armor += armor.armor;

            // And without ring --
            if fight(me.clone(), boss.clone()) {
                mini_win = std::cmp::min(mini_win, cost);
            } else {
                maxi_loose = std::cmp::max(maxi_loose, cost);
            }

            // Or each ring --
            for ring in rings.iter() {
                let mut me = me.clone();
                let mut cost = cost;

                cost += ring.cost;
                me.armor += ring.armor;
                me.damage += ring.damage;

                if fight(me.clone(), boss.clone()) {
                    mini_win = std::cmp::min(mini_win, cost);
                } else {
                    maxi_loose = std::cmp::max(maxi_loose, cost);
                }
            }

            // Or each combinations of two rings --
            for combination in rings.iter().combinations(2) {
                let mut me = me.clone();
                let mut cost = cost;

                cost += combination[0].cost;
                me.armor += combination[0].armor;
                me.damage += combination[0].damage;
                cost += combination[1].cost;
                me.armor += combination[1].armor;
                me.damage += combination[1].damage;

                if fight(me.clone(), boss.clone()) {
                    mini_win = std::cmp::min(mini_win, cost);
                } else {
                    maxi_loose = std::cmp::max(maxi_loose, cost);
                }
            }
        }
    }

    println!("Part one: {}", mini_win);
    println!("Part two: {}", maxi_loose);
}

fn fight(mut me: Player, mut boss: Player) -> bool {
    while me.hit_points > 0 && boss.hit_points > 0 {
        let damage = if me.damage > boss.armor {
            me.damage - boss.armor
        } else {
            1
        };

        boss.hit_points -= damage;
        if boss.hit_points <= 0 {
            return true;
        }

        let damage = if boss.damage > me.armor {
            boss.damage - me.armor
        } else {
            1
        };

        me.hit_points -= damage;
    }
    false
}

fn parse_line(line: &str) -> Item {
    let mut it = line.split_whitespace().rev();

    let armor = it.next().unwrap().parse::<i32>().unwrap();
    let damage = it.next().unwrap().parse::<i32>().unwrap();
    let cost = it.next().unwrap().parse::<i32>().unwrap();

    Item::new(cost, damage, armor)
}
