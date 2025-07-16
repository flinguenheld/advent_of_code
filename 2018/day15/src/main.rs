use owo_colors::OwoColorize;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Player {
    row: i32,
    col: i32,

    hit_points: i32,
    attack: i32,
    elf: bool,
}

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";
    // let path = "input_example_2.txt";

    let input = std::fs::read_to_string(path).unwrap();

    let map: HashSet<(i32, i32)> = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.char_indices()
                .filter(|(_, c)| *c != '#')
                .map(move |(col, _)| (row as i32, col as i32))
        })
        .collect();

    let players: Vec<Player> = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.char_indices()
                .filter(|(_, c)| ['G', 'E'].contains(c))
                .map(move |(col, c)| Player {
                    row: row as i32,
                    col: col as i32,
                    hit_points: 200,
                    attack: 3,
                    elf: c == 'E',
                })
        })
        .collect();
    // print(&map, &players);

    let (players_done, rounds) = play(players.clone(), &map, false);
    println!(
        "Part one: {}",
        players_done.iter().fold(0, |acc, p| acc + p.hit_points) * (rounds - 1)
    );

    let mut attack = 0;
    loop {
        attack += 1;
        let mut players = players.clone();
        players
            .iter_mut()
            .filter(|p| p.elf)
            .for_each(|p| p.attack = attack);

        let (players_done, rounds) = play(players, &map, true);

        if !players_done.is_empty() {
            println!(
                "Part two: {}",
                players_done.iter().fold(0, |acc, p| acc + p.hit_points) * (rounds - 1)
            );
            break;
        }
    }
}

fn play(mut players: Vec<Player>, map: &HashSet<(i32, i32)>, part_two: bool) -> (Vec<Player>, i32) {
    let mut round = 0;
    loop {
        if part_two && players.iter().any(|p| p.elf && p.hit_points <= 0) {
            return (Vec::new(), 0);
        }

        // Get players with their order
        players.retain(|p| p.hit_points > 0);
        players.sort_by_key(|p| p.col);
        players.sort_by_key(|p| p.row);

        for i in 0..players.len() {
            if players[i].hit_points <= 0 {
                continue;
            }

            if !try_to_attack(&mut players, i) {
                // MOVE !
                // Where is the closest ennemy ?
                // Get paths to the closest enemies
                let paths = get_paths(
                    (players[i].row, players[i].col),
                    map,
                    players
                        .iter()
                        .filter(|p| p.elf != players[i].elf && p.hit_points > 0)
                        .collect(),
                    players
                        .iter()
                        .filter(|p| p.elf == players[i].elf && p.hit_points > 0)
                        .collect(),
                );

                // Combine paths & players to get the potential targets
                let mut closest_enemy: Vec<((i32, i32), i32)> = players
                    .iter()
                    .filter(|p| {
                        p.elf != players[i].elf
                            && paths.contains_key(&(p.row, p.col))
                            && p.hit_points > 0
                    })
                    .map(|p| ((p.row, p.col), *paths.get(&(p.row, p.col)).unwrap()))
                    .collect();

                if !closest_enemy.is_empty() {
                    let (_, min) = *closest_enemy.iter().min_by_key(|e| e.1).unwrap();
                    closest_enemy.retain(|e| e.1 == min);

                    // Sort to respect the reading order
                    closest_enemy.sort_by_key(|((_, col), _)| *col);
                    closest_enemy.sort_by_key(|((row, _), _)| *row);

                    // Now we have the one
                    if let Some((target, _)) = closest_enemy.first_mut() {
                        // So redo a path from the target to the current
                        // It allows me to know the possible moves
                        let mut path = get_paths(
                            *target,
                            map,
                            vec![&players[i]],
                            players
                                .iter()
                                .filter(|p| (p.row, p.col) != *target && p.hit_points > 0)
                                .collect(),
                        );

                        // We can only move in these directions
                        let neighbours: Vec<(i32, i32)> = [(0, 1), (1, 0), (0, -1), (-1, 0)]
                            .iter()
                            .map(|p| (players[i].row + p.0, players[i].col + p.1))
                            .collect();

                        // Only keep neighbours
                        path.retain(|pt, _| neighbours.contains(pt));

                        // Only keep mini
                        let min = *path.values().min().unwrap();
                        path.retain(|_, v| *v == min);

                        let mut cache: Vec<&(i32, i32)> = path.keys().collect();

                        // Sort to respect the reading order
                        cache.sort_by_key(|(_, col)| col);
                        cache.sort_by_key(|(row, _)| row);

                        if let Some(next) = cache.first() {
                            if !players.iter().any(|p| (p.row, p.col) == **next) {
                                players[i].row = next.0;
                                players[i].col = next.1;
                            }
                        }

                        try_to_attack(&mut players, i);
                    }
                }
            }

            if players.iter().filter(|p| p.elf != players[i].elf).count() == 0 {
                return (players, round);
            }
        }

        round += 1;
    }
}

fn try_to_attack(players: &mut [Player], who: usize) -> bool {
    // Is already close to an enemy ?
    let neighbours: Vec<(i32, i32)> = [(-1, 0), (0, 1), (1, 0), (0, -1)]
        .iter()
        .map(|(r, c)| (players[who].row + r, players[who].col + c))
        .collect();

    let mut nearby_enemies: Vec<&Player> = players
        .iter()
        .filter(|p| {
            neighbours.contains(&(p.row, p.col)) && p.elf != players[who].elf && p.hit_points > 0
        })
        .collect();

    if !nearby_enemies.is_empty() {
        // Get the weakest enemy (that's mean)
        let min = nearby_enemies
            .iter()
            .min_by_key(|v| v.hit_points)
            .unwrap()
            .hit_points;
        nearby_enemies.retain(|e| e.hit_points == min);
        nearby_enemies.sort_by_key(|p| p.col);
        nearby_enemies.sort_by_key(|p| p.row);

        if let Some(victim_idx) = players
            .iter()
            .position(|p| *p == **nearby_enemies.first().unwrap())
        {
            players[victim_idx].hit_points -= players[who].attack;
            return true;
        }
    }
    false
}

fn get_paths(
    start: (i32, i32),
    map: &HashSet<(i32, i32)>,
    targets: Vec<&Player>,
    exclude: Vec<&Player>,
) -> HashMap<(i32, i32), i32> {
    let mut cache = HashMap::new();

    // neighbours with the wording order
    let neighbours = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut queue = VecDeque::from([(start, 0)]);
    let mut done = i32::MAX;

    while let Some((current, value)) = queue.pop_front() {
        cache.insert(current, value);

        for neighbour in neighbours.iter() {
            let new = (current.0 + neighbour.0, current.1 + neighbour.1);

            if !queue.iter().any(|(pt, _)| *pt == new)
                && !cache.contains_key(&new)
                && map.contains(&new)
                && value <= done
                && !exclude.iter().any(|p| new == (p.row, p.col))
            {
                if targets.iter().any(|p| new == (p.row, p.col)) {
                    // Do not add in the queue !
                    cache.insert(new, value + 1);
                    done = value;
                } else {
                    queue.push_back((new, value + 1));
                }
            }
        }
    }

    cache
}

#[allow(dead_code)]
fn print(map: &HashSet<(i32, i32)>, players: &[((i32, i32), Player)]) {
    let min_row = map.iter().min_by_key(|(row, _)| row).unwrap().0;
    let max_row = map.iter().max_by_key(|(row, _)| row).unwrap().0;

    let min_col = map.iter().min_by_key(|(_, col)| col).unwrap().1;
    let max_col = map.iter().max_by_key(|(_, col)| col).unwrap().1;

    for row in min_row - 1..max_row + 2 {
        for col in min_col - 1..max_col + 2 {
            if let Some((_, player)) = players.iter().find(|((r, c), _)| *r == row && *c == col) {
                if player.elf {
                    print!("{}", "E".green());
                } else {
                    print!("{}", "G".red());
                }
            } else if map.contains(&(row, col)) {
                print!("{}", ".".bright_black());
            } else {
                print!("{}", "#".black().on_black());
            }
        }
        println!();
    }
}

#[allow(dead_code)]
fn print_paths(map: &HashMap<(i32, i32), i32>, players: &[((i32, i32), Player)]) {
    let min_row = map.keys().min_by_key(|(row, _)| row).unwrap().0;
    let max_row = map.keys().max_by_key(|(row, _)| row).unwrap().0;

    let min_col = map.keys().min_by_key(|(_, col)| col).unwrap().1;
    let max_col = map.keys().max_by_key(|(_, col)| col).unwrap().1;

    for row in min_row - 1..max_row + 2 {
        for col in min_col - 1..max_col + 2 {
            if let Some((_, player)) = players.iter().find(|((r, c), _)| *r == row && *c == col) {
                if player.elf {
                    print!("{}", "E".bright_cyan());
                } else {
                    print!("{}", "G".red());
                }
            } else if let Some(value) = map.get(&(row, col)) {
                print!("{}", (value % 10).white());
            } else {
                print!("{}", "#".black().on_black());
            }
        }
        println!();
    }
}
