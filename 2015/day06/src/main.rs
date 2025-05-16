fn main() {
    let mut grid = vec![vec![false; 1000]; 1000];
    let mut grid_2 = vec![vec![0_u32; 1000]; 1000];

    for line in std::fs::read_to_string("input.txt").unwrap().lines() {
        if let Some((from, to)) = line.split_once(" through ") {
            let (from_row, from_col) = from.split_once(',').unwrap();
            let from_row = from_row
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse::<usize>()
                .unwrap();
            let from_col = from_col.parse::<usize>().unwrap();

            let (to_row, to_col) = to.split_once(',').unwrap();
            let to_row = to_row.parse::<usize>().unwrap();
            let to_col = to_col.parse::<usize>().unwrap();

            for row in from_row..=to_row {
                for col in from_col..=to_col {
                    if line.starts_with("toggle") {
                        grid[row][col] = !grid[row][col];
                        grid_2[row][col] += 2;
                    } else if line.starts_with("turn on") {
                        grid[row][col] = true;
                        grid_2[row][col] += 1;
                    } else {
                        grid[row][col] = false;
                        grid_2[row][col] = grid_2[row][col].saturating_sub(1);
                    }
                }
            }
        }
    }

    println!(
        "Part one: {}",
        grid.iter().flatten().filter(|r| **r).count()
    );

    println!("Part two: {}", grid_2.iter().flatten().sum::<u32>());
}
