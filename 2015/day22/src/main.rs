const SPELLS: [Spell; 5] = [
    Spell::new(53, 4, 0, 0, 0, 0),    // Missile
    Spell::new(73, 2, 0, 0, 2, 0),    // Drain
    Spell::new(113, 0, 6, 0, 0, 7),   // Shield
    Spell::new(173, 3, 6, 0, 0, 0),   // Poison
    Spell::new(229, 0, 5, 101, 0, 0), // Recharge
];

#[derive(Debug, Clone)]
struct Player {
    hit_points: i32,
    damage: i32,
    armor: i32,
    mana: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Spell {
    cost: i32,
    damage: i32,
    turns: i32,
    recharge: i32,
    heal: i32,
    armor: i32,
}

impl Spell {
    #[rustfmt::skip]
    const fn new( cost    : i32, damage: i32, turns: i32,
                  recharge: i32, heal  : i32, armor: i32, ) -> Self {
        Spell { cost, damage, turns, recharge, heal, armor }
    }
}

fn main() {
    let mut me = Player {
        hit_points: 50,
        damage: 0,
        armor: 0,
        mana: 500,
    };
    let mut boss = Player {
        hit_points: 55,
        damage: 8,
        armor: 0,
        mana: 0,
    };

    // Example 1       -> Result: 226
    // me.hit_points = 10;
    // me.mana = 250;
    // boss.hit_points = 13;

    // Example 2       -> Result: 614
    // me.hit_points = 10;
    // me.mana = 250;
    // boss.hit_points = 14;

    println!(
        "Part one: {}",
        go(me.clone(), boss.clone(), Vec::new(), 0, i32::MAX, false)
    );
    println!("Part two: {}", go(me, boss, Vec::new(), 0, i32::MAX, true));
}

fn go(
    me: Player,
    boss: Player,
    current_spells: Vec<Spell>,
    current_amount: i32,
    mut mini: i32,
    hard: bool,
) -> i32 {
    if me.mana >= SPELLS.iter().min_by_key(|sp| sp.cost).unwrap().cost {
        for new_spell in SPELLS.iter() {
            if current_amount >= mini {
                return i32::MAX;
            }

            let mut player = me.clone();
            let mut boss = boss.clone();
            let mut current_spells = current_spells.clone();
            let mut current_amount = current_amount;

            // Player turn --
            if hard {
                player.hit_points -= 1;
                if player.hit_points <= 0 {
                    continue;
                }
            }

            current_spells = apply_spells(&mut player, &mut boss, current_spells);

            if current_spells.iter().all(|s| s.cost != new_spell.cost)
                && new_spell.cost <= player.mana
            {
                // Add or apply now the new spell and its cost
                player.mana -= new_spell.cost;
                current_amount += new_spell.cost;

                if new_spell.turns == 0 {
                    apply_spells(&mut player, &mut boss, vec![new_spell.clone()]);
                } else {
                    current_spells.push(new_spell.clone());
                }

                if boss.hit_points <= 0 {
                    return current_amount;
                }

                // Boss turn --
                current_spells = apply_spells(&mut player, &mut boss, current_spells);

                if boss.hit_points <= 0 {
                    return current_amount;
                }

                if boss.damage > player.armor {
                    player.hit_points -= boss.damage - player.armor;
                } else {
                    player.hit_points -= 1;
                }

                if player.hit_points <= 0 {
                    continue;
                }

                // --
                mini = std::cmp::min(
                    mini,
                    go(player, boss, current_spells, current_amount, mini, hard),
                );
            }
        }
    }
    mini
}

fn apply_spells(me: &mut Player, boss: &mut Player, mut current_spells: Vec<Spell>) -> Vec<Spell> {
    me.armor = 0;

    for spell in current_spells.iter_mut() {
        me.hit_points += spell.heal;
        me.mana += spell.recharge;

        if spell.armor > 0 {
            me.armor = spell.armor;
        }

        boss.hit_points -= spell.damage;
        spell.turns -= 1;
    }

    current_spells.retain(|s| s.turns > 0);
    current_spells
}
