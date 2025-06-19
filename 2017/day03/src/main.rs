use std::collections::HashMap;

const NEIGHBOURS: [(i32, i32); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

fn main() {
    // let input = 12;
    // let input = 1024;
    let input = 347991;

    // Get the first value of the next square

    // 17  16  15  14  13
    // 18   5   4   3  12
    // 19   6   1   2  11  28
    // 20   7   8   9  10  27
    // 21  22  23  24  25  26

    let mut cur_width = 3;
    let mut cur_value = 2;

    let mut prev_width = 0;
    let mut prev_value = 0;

    while cur_value < input {
        prev_width = cur_width;
        prev_value = cur_value;

        cur_value += 2 * cur_width + (cur_width - 2) * 2 + 1;
        cur_width += 2;
    }

    // Manhattan distance is absolute so the value up and down cyclicly
    let min = prev_width / 2;
    let max = prev_width - 1;

    let mut current = min;
    let mut up = true;
    for _ in 0..input - prev_value {
        current += match up {
            true => 1,
            false => -1,
        };

        if current == max || current == min {
            up = !up;
        }
    }
    println!("Part one: {}", current);

    // --
    let mut grid: HashMap<(i32, i32), i32> = HashMap::new();
    grid.insert((0, 0), 1);
    grid.insert((1, 0), 1);

    let mut x = 1;
    let mut y = 0;

    let mut i = 1;
    let mut last = 1;

    // The top value of the current square matches the loop index i
    // And it's the same value for x/y in positive/negative
    // So loop in each way until it reaches the value and add all points
    //
    // 147  142  133  122      59 |i
    // 304    5    4    2 |i   57 |
    // 330   10    1    1 |    54 |
    // 351   11   23   25      26
    // 362  747  806--->   ...
    //

    while last < input {
        while last < input && y != i {
            y += 1;
            last = sum_neighbours(&grid, (x, y));
            grid.insert((x, y), last);
        }

        while last < input && x != -i {
            x -= 1;
            last = sum_neighbours(&grid, (x, y));
            grid.insert((x, y), last);
        }

        while last < input && y != -i {
            y -= 1;
            last = sum_neighbours(&grid, (x, y));
            grid.insert((x, y), last);
        }

        while last < input && x != i + 1 {
            x += 1;
            last = sum_neighbours(&grid, (x, y));
            grid.insert((x, y), last);
        }

        i += 1;
    }

    println!("Part two: {}", last);
}

fn sum_neighbours(grid: &HashMap<(i32, i32), i32>, point: (i32, i32)) -> i32 {
    let mut total = 0;
    for neighbour in NEIGHBOURS.iter() {
        if let Some(n) = grid.get(&(point.0 + neighbour.0, point.1 + neighbour.1)) {
            total += n;
        }
    }

    total
}
