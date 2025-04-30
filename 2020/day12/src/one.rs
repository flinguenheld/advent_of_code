use std::fs;

const DIR: [char; 4] = ['n', 'e', 's', 'w'];

#[derive(Default)]
struct Ship {
    direction: usize,
    north_south: i32,
    east_west: i32,
}

impl Ship {
    fn rotate_right(&mut self, degrees: i32) {
        self.direction += degrees as usize / 90;
        self.direction = self.direction.rem_euclid(DIR.len());
    }
    fn rotate_left(&mut self, degrees: i32) {
        let d = self.direction as i32 - degrees / 90;
        self.direction = d.rem_euclid(DIR.len() as i32) as usize;
    }

    fn add_forward(&mut self, value: i32) {
        match DIR[self.direction] {
            'n' => self.north_south += value,
            's' => self.north_south -= value,
            'e' => self.east_west += value,
            _ => self.east_west -= value,
        }
    }
}

fn main() {
    // let path = "input_example.txt";
    let path = "input.txt";

    let input = fs::read_to_string(path).unwrap();
    let mut ship = Ship {
        direction: 1,
        ..Default::default()
    };

    for line in input.lines() {
        let (action, value) = line.split_at(1);

        if let Ok(value) = value.parse::<i32>() {
            match action {
                "N" => ship.north_south += value,
                "S" => ship.north_south -= value,
                "E" => ship.east_west += value,
                "W" => ship.east_west -= value,
                "L" => ship.rotate_left(value),
                "R" => ship.rotate_right(value),
                _ => ship.add_forward(value),
            }
        }
    }

    println!(
        "Part one: {}",
        ship.north_south.abs() + ship.east_west.abs()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turn_around_part_one() {
        let mut ship = Ship {
            direction: 1,
            ..Default::default()
        };

        ship.rotate_right(90);
        assert!(ship.direction == 2);
        ship.rotate_right(90);
        assert!(ship.direction == 3);
        ship.rotate_right(90);
        assert!(ship.direction == 0);
        ship.rotate_right(90);
        assert!(ship.direction == 1);
        ship.rotate_right(90);
        assert!(ship.direction == 2);
        ship.rotate_right(180);
        assert!(ship.direction == 0);
        ship.rotate_left(180);
        assert!(ship.direction == 2);
        ship.rotate_left(90);
        assert!(ship.direction == 1);
        ship.rotate_left(90);
        assert!(ship.direction == 0);
        ship.rotate_left(90);
        assert!(ship.direction == 3);
        ship.rotate_left(270);
        assert!(ship.direction == 0);
    }
}
