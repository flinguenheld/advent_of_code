use std::collections::VecDeque;

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let grid: Vec<Vec<u8>> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| l.chars().map(|c| c as u8 - 48).collect())
        .collect();

    let mut count = 0;
    for (r, row) in grid.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            if get_neighbours(r, c, &grid)
                .iter()
                .all(|(nr, nc)| grid[*nr][*nc] > *col)
            {
                count += *col as u32 + 1;
            }
        }
    }

    println!("Part one: {}", count);

    // --
    let mut cache: Vec<Vec<(usize, usize)>> = Vec::new();

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            let current_start = (row, col);

            if grid[row][col] < 9 && cache.iter().flatten().all(|pt| *pt != current_start) {
                let mut new_bassin: Vec<(usize, usize)> = Vec::new();

                let mut queue = VecDeque::from([(row, col)]);
                while let Some(point) = queue.pop_front() {
                    new_bassin.push(point);

                    for (nr, nc) in get_neighbours(point.0, point.1, &grid) {
                        if grid[nr][nc] < 9
                            && !queue.contains(&(nr, nc))
                            && !new_bassin.contains(&(nr, nc))
                        {
                            queue.push_back((nr, nc));
                        }
                    }
                }

                cache.push(new_bassin);
            }
        }
    }

    cache.sort_by_key(|b| b.len());
    // print(&grid, &cache);

    let mut it = cache.iter().rev();
    println!(
        "Part two: {}",
        it.next().unwrap().len() * it.next().unwrap().len() * it.next().unwrap().len()
    );
}

#[allow(dead_code)]
fn print(grid: &[Vec<u8>], cache: &[Vec<(usize, usize)>]) {
    for (r, row) in grid.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            if *col == 9 {
                print!(".");
            } else if cache.iter().flatten().any(|pt| *pt == (r, c)) {
                print!("c");
            } else {
                print!("{}", col);
            }
        }
        println!();
    }
}

fn get_neighbours(row: usize, col: usize, grid: &[Vec<u8>]) -> Vec<(usize, usize)> {
    let mut neigbours = Vec::new();

    if row > 0 {
        neigbours.push((row - 1, col));
    }
    if row < grid.len() - 1 {
        neigbours.push((row + 1, col));
    }
    if col > 0 {
        neigbours.push((row, col - 1));
    }
    if col < grid[0].len() - 1 {
        neigbours.push((row, col + 1));
    }

    neigbours
}
