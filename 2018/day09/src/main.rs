use std::collections::VecDeque;

fn main() {
    // let (nb_players, nb_marbles) = (9, 25);
    // let (nb_players, nb_marbles) = (10, 1618);
    // let (nb_players, nb_marbles) = (13, 7999);
    // let (nb_players, nb_marbles) = (17, 1104);
    // let (nb_players, nb_marbles) = (21, 6111);
    // let (nb_players, nb_marbles) = (30, 5807);

    let (nb_players, nb_marbles) = (431, 70_950);

    println!("Part one: {:?}", play(nb_players, nb_marbles));

    let (nb_players, nb_marbles) = (431, 7_095_000);
    println!("Part two: {:?}", play(nb_players, nb_marbles));
}

fn play(nb_players: usize, nb_marbles: u64) -> u64 {
    let mut marbles: VecDeque<u64> = VecDeque::from([0]);

    let mut scores = vec![0; nb_players];
    let mut player = 0;

    for i in 1..=nb_marbles {
        player = (player + 1) % nb_players;

        if i % 23 == 0 {
            marbles.rotate_right(7);
            if let Some(v) = marbles.pop_back() {
                scores[player] += i + v;
            }
            marbles.rotate_left(1);
        } else {
            marbles.rotate_left(1);
            marbles.push_back(i);
        }
    }

    *scores.iter().max().unwrap()
}
