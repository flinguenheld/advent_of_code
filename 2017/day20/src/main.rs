#[derive(Debug, Clone)]
struct Particle {
    p: (i64, i64, i64),
    v: (i64, i64, i64),
    a: (i64, i64, i64),

    who: usize,
}

impl Particle {
    fn go(&mut self) {
        self.v.0 += self.a.0;
        self.v.1 += self.a.1;
        self.v.2 += self.a.2;

        self.p.0 += self.v.0;
        self.p.1 += self.v.1;
        self.p.2 += self.v.2;
    }
    fn distance(&self) -> i64 {
        self.p.0.abs() + self.p.1.abs() + self.p.2.abs()
    }
}

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let input: Vec<Particle> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let fields: Vec<&str> = line.split(", ").collect();

            Particle {
                p: extract(fields[0]),
                v: extract(fields[1]),
                a: extract(fields[2]),
                who: i,
            }
        })
        .collect();

    let mut particles = input.clone();
    let mut no_change_for = 0;
    let mut mini = 0;
    while no_change_for < 200 {
        particles.iter_mut().for_each(|line| line.go());

        let new_mini = particles
            .iter()
            .min_by_key(|elem| elem.distance())
            .unwrap()
            .who;

        if new_mini == mini {
            no_change_for += 1;
        } else {
            mini = new_mini;
            no_change_for = 0;
        }
    }

    println!("Part one: {}", mini);

    // --
    let mut particles = input.clone();
    let mut no_change_for = 0;
    let mut len = 0;
    while no_change_for < 10 {
        particles.iter_mut().for_each(|line| line.go());

        let mut collided = Vec::new();
        for a in particles.iter() {
            if particles
                .iter()
                .filter(|b| a.who != b.who)
                .any(|b| a.p == b.p)
            {
                collided.push(a.who);
            }
        }

        particles.retain(|p| !collided.contains(&p.who));

        if len == particles.len() {
            no_change_for += 1;
        } else {
            len = particles.len();
            no_change_for = 0;
        }
    }

    println!("Part two: {}", len);
}

fn extract(fields: &str) -> (i64, i64, i64) {
    let field: Vec<&str> = fields.split(',').collect();
    (clean(field[0]), clean(field[1]), clean(field[2]))
}
fn clean(num: &str) -> i64 {
    num.chars()
        .filter(|c| c.is_ascii_digit() || *c == '-')
        .collect::<String>()
        .parse::<i64>()
        .unwrap()
}
