use std::{fs, mem::swap};

#[derive(Default)]
struct Position {
    north_south: i64,
    east_west: i64,
}

impl Position {
    fn rotate_right(&mut self, degrees: i64) {
        for _ in 0..(degrees / 90) {
            match (self.east_west > 0, self.north_south > 0) {
                (true, true) => {
                    swap(&mut self.east_west, &mut self.north_south);
                    self.east_west = self.east_west.abs();
                    self.north_south = -self.north_south.abs();
                }
                (true, false) => {
                    swap(&mut self.east_west, &mut self.north_south);
                    self.east_west = -self.east_west.abs();
                    self.north_south = -self.north_south.abs();
                }
                (false, false) => {
                    swap(&mut self.east_west, &mut self.north_south);
                    self.east_west = -self.east_west.abs();
                    self.north_south = self.north_south.abs();
                }
                (false, true) => {
                    swap(&mut self.east_west, &mut self.north_south);
                    self.east_west = self.east_west.abs();
                    self.north_south = self.north_south.abs();
                }
            }
        }
    }

    fn rotate_left(&mut self, degrees: i64) {
        for _ in 0..(degrees / 90) {
            match (self.east_west > 0, self.north_south > 0) {
                (true, true) => {
                    swap(&mut self.east_west, &mut self.north_south);
                    self.east_west = -self.east_west.abs();
                    self.north_south = self.north_south.abs();
                }
                (false, true) => {
                    swap(&mut self.east_west, &mut self.north_south);
                    self.east_west = -self.east_west.abs();
                    self.north_south = -self.north_south.abs();
                }
                (false, false) => {
                    swap(&mut self.east_west, &mut self.north_south);
                    self.east_west = self.east_west.abs();
                    self.north_south = -self.north_south.abs();
                }
                (true, false) => {
                    swap(&mut self.east_west, &mut self.north_south);
                    self.east_west = self.east_west.abs();
                    self.north_south = self.north_south.abs();
                }
            }
        }
    }
}

fn main() {
    // let path = "input_example.txt";
    let path = "input.txt";

    let input = fs::read_to_string(path).unwrap();
    let mut ship = Position {
        east_west: 0,
        north_south: 0,
    };
    let mut waypoint = Position {
        east_west: -10,
        north_south: 1,
    };

    for line in input.lines() {
        let (action, value) = line.split_at(1);

        if let Ok(value) = value.parse::<i64>() {
            match action {
                "N" => waypoint.north_south += value,
                "S" => waypoint.north_south -= value,
                "E" => waypoint.east_west -= value,
                "W" => waypoint.east_west += value,
                "L" => waypoint.rotate_right(value),
                "R" => waypoint.rotate_left(value),
                _ => {
                    ship.north_south += value * waypoint.north_south;
                    ship.east_west += value * waypoint.east_west;
                }
            }
        }
    }

    println!(
        "Part two: {}",
        ship.north_south.abs() + ship.east_west.abs()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turn_around_part_2() {
        let mut waypoint = Position {
            east_west: 5,
            north_south: 10,
        };

        waypoint.rotate_right(90);
        assert!((waypoint.east_west, waypoint.north_south) == (10, -5));
        waypoint.rotate_right(90);
        assert!((waypoint.east_west, waypoint.north_south) == (-5, -10));
        waypoint.rotate_right(90);
        assert!((waypoint.east_west, waypoint.north_south) == (-10, 5));
        waypoint.rotate_right(90);
        assert!((waypoint.east_west, waypoint.north_south) == (5, 10));
        waypoint.rotate_right(180);
        assert!((waypoint.east_west, waypoint.north_south) == (-5, -10));
        waypoint.rotate_right(360);
        assert!((waypoint.east_west, waypoint.north_south) == (-5, -10));
        waypoint.rotate_right(450);
        assert!((waypoint.east_west, waypoint.north_south) == (-10, 5));

        waypoint.rotate_left(90);
        assert!((waypoint.east_west, waypoint.north_south) == (-5, -10));
        waypoint.rotate_left(90);
        assert!((waypoint.east_west, waypoint.north_south) == (10, -5));
        waypoint.rotate_left(90);
        assert!((waypoint.east_west, waypoint.north_south) == (5, 10));
        waypoint.rotate_left(90);
        assert!((waypoint.east_west, waypoint.north_south) == (-10, 5));
        waypoint.rotate_left(360);
        assert!((waypoint.east_west, waypoint.north_south) == (-10, 5));
        waypoint.rotate_left(450);
        assert!((waypoint.east_west, waypoint.north_south) == (-5, -10));
    }
}
