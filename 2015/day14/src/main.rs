#[derive(Debug)]
struct Reindeer {
    name: String,
    speed: i32,
    flying_time: i32,
    rest: i32,

    counter: i32,
    current_position: i32,
    points: i32,
}

impl Reindeer {
    fn new(name: &str, speed: &str, time: &str, rest: &str) -> Self {
        Reindeer {
            name: name.to_string(),
            speed: speed.parse::<i32>().unwrap(),
            flying_time: time.parse::<i32>().unwrap(),
            rest: -rest.parse::<i32>().unwrap(),

            counter: time.parse::<i32>().unwrap(),
            current_position: 0,
            points: 0,
        }
    }

    fn fly(&mut self) {
        self.counter -= 1;

        if self.counter >= 0 {
            self.current_position += self.speed;
        } else if self.counter <= self.rest {
            self.counter = self.flying_time;
        }
    }
}

fn main() {
    // let (path, seconds) = ("input_example.txt", 1000);
    let (path, seconds) = ("input.txt", 2503);

    let mut reindeers: Vec<Reindeer> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            let mut it = line.split_whitespace();
            Reindeer::new(
                it.next().unwrap(),
                it.nth(2).unwrap(),
                it.nth(2).unwrap(),
                it.nth(6).unwrap(),
            )
        })
        .collect();

    for _ in 0..seconds {
        reindeers.iter_mut().for_each(|r| r.fly());

        // Points --
        let top_of_the_race = reindeers
            .iter_mut()
            .max_by(|a, b| a.current_position.cmp(&b.current_position))
            .unwrap()
            .current_position;

        reindeers
            .iter_mut()
            .filter(|r| r.current_position == top_of_the_race)
            .for_each(|r| r.points += 1);
    }

    let winner = reindeers
        .iter()
        .max_by(|a, b| a.current_position.cmp(&b.current_position))
        .unwrap();
    println!(
        "Part one: {} is the winner with {} km.",
        winner.name, winner.current_position
    );

    let winner = reindeers
        .iter()
        .max_by(|a, b| a.points.cmp(&b.points))
        .unwrap();
    println!(
        "Part two: {} is the winner with {} points.",
        winner.name, winner.points
    );
}
