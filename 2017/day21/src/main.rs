use std::collections::HashMap;

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let mut rules: HashMap<Vec<Vec<char>>, Vec<Vec<char>>> = HashMap::new();
    for line in std::fs::read_to_string(path).unwrap().lines() {
        // Get the match and save all flipped/rotated values
        if let Some((left, right)) = line.split_once(" => ") {
            let to: Vec<Vec<char>> = right.split('/').map(|c| c.chars().collect()).collect();
            let mut key: Vec<Vec<char>> = left.split('/').map(|c| c.chars().collect()).collect();

            for _ in 0..4 {
                rules.insert(key.clone(), to.clone());
                rules.insert(flip_h(&key), to.clone());
                rules.insert(flip_v(&key), to.clone());
                key = rotate(&key);
            }
        }
    }

    let base = vec![
        vec!['.', '#', '.'],
        vec!['.', '.', '#'],
        vec!['#', '#', '#'],
    ];

    let mut grid = base.clone();

    for _ in 0..5 {
        let mut temp = if grid.len().rem_euclid(2) == 0 {
            divide_2(&grid)
        } else {
            divide_3(&grid)
        };

        // Replace all vecs with the rule
        for to_up in temp.iter_mut().flatten() {
            *to_up = rules.get(to_up).unwrap().clone();
        }

        // And merge
        grid = merge(&temp);
    }

    println!(
        "Part one: {}",
        grid.iter().flatten().filter(|c| **c == '#').count()
    );

    // --
    let mut grid = base.clone();

    for _ in 0..18 {
        let mut temp = if grid.len().rem_euclid(2) == 0 {
            divide_2(&grid)
        } else {
            divide_3(&grid)
        };

        // Replace all vecs with the rule
        for to_up in temp.iter_mut().flatten() {
            *to_up = rules.get(to_up).unwrap().clone();
        }

        // And merge
        grid = merge(&temp);
    }

    println!(
        "Part two: {}",
        grid.iter().flatten().filter(|c| **c == '#').count()
    );
}

fn merge(grid: &[Vec<Vec<Vec<char>>>]) -> Vec<Vec<char>> {
    let mut merged: Vec<Vec<char>> = Vec::new();

    for big_row in grid.iter() {
        for col in 0..big_row[0].len() {
            let mut new_row = Vec::new();
            for blah in big_row.iter() {
                new_row.extend(blah[col].iter());
            }
            merged.push(new_row);
        }
    }

    merged
}

fn divide_2(grid: &[Vec<char>]) -> Vec<Vec<Vec<Vec<char>>>> {
    let nb = grid.len() / 2;

    let mut temp: Vec<Vec<Vec<Vec<char>>>> =
        vec![vec![vec![vec!['0', '1'], vec!['2', '3']]; nb]; nb];

    for (big_row, r) in temp.iter_mut().enumerate() {
        for (big_col, c) in r.iter_mut().enumerate() {
            c[0][0] = grid[big_row * 2][big_col * 2];
            c[0][1] = grid[big_row * 2][big_col * 2 + 1];

            c[1][0] = grid[big_row * 2 + 1][big_col * 2];
            c[1][1] = grid[big_row * 2 + 1][big_col * 2 + 1];
        }
    }

    temp
}

fn divide_3(grid: &[Vec<char>]) -> Vec<Vec<Vec<Vec<char>>>> {
    let nb = grid.len() / 3;
    let mut temp: Vec<Vec<Vec<Vec<char>>>> = vec![
        vec![
            vec![
                vec!['0', '1', '2'],
                vec!['2', '3', '4'],
                vec!['5', '6', '7']
            ];
            nb
        ];
        nb
    ];

    for (big_row, r) in temp.iter_mut().enumerate() {
        for (big_col, c) in r.iter_mut().enumerate() {
            c[0][0] = grid[big_row * 3][big_col * 3];
            c[0][1] = grid[big_row * 3][big_col * 3 + 1];
            c[0][2] = grid[big_row * 3][big_col * 3 + 2];

            c[1][0] = grid[big_row * 3 + 1][big_col * 3];
            c[1][1] = grid[big_row * 3 + 1][big_col * 3 + 1];
            c[1][2] = grid[big_row * 3 + 1][big_col * 3 + 2];

            c[2][0] = grid[big_row * 3 + 2][big_col * 3];
            c[2][1] = grid[big_row * 3 + 2][big_col * 3 + 1];
            c[2][2] = grid[big_row * 3 + 2][big_col * 3 + 2];
        }
    }

    temp
}

#[allow(dead_code)]
fn print(grid: &[Vec<char>]) {
    for row in grid.iter() {
        for col in row.iter() {
            print!("{}", col);
        }
        println!();
    }
}

fn rotate(grid: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut rotated = grid.to_owned();
    let mut to_col = grid.len();

    for row in grid.iter() {
        to_col -= 1;
        for (to_row, case) in row.iter().enumerate() {
            rotated[to_row][to_col] = *case;
        }
    }
    rotated
}

fn flip_h(grid: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut i = 0;
    let mut j = grid.len() - 1;
    let mut flipped = grid.to_owned();
    while i < j {
        flipped.swap(i, j);
        i += 1;
        j = j.saturating_sub(1);
    }
    flipped
}

fn flip_v(grid: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut i = 0;
    let mut j = grid.len() - 1;
    let mut flipped = grid.to_owned();
    while i < j {
        for row in flipped.iter_mut() {
            row.swap(i, j);
        }
        i += 1;
        j = j.saturating_sub(1);
    }
    flipped
}

// --------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn rotate_2() {
        let grid = vec![vec!['1', '2'], vec!['3', '4']];
        let expect = vec![vec!['3', '1'], vec!['4', '2']];
        assert_eq!(rotate(&grid), expect);
    }
    #[test]
    fn rotate_3() {
        let grid = vec![
            vec!['1', '2', '3'],
            vec!['4', '5', '6'],
            vec!['7', '8', '9'],
        ];
        let expect = vec![
            vec!['7', '4', '1'],
            vec!['8', '5', '2'],
            vec!['9', '6', '3'],
        ];
        assert_eq!(rotate(&grid), expect);
    }

    #[test]
    fn flip_h_2() {
        let grid = vec![vec!['1', '2'], vec!['3', '4']];
        let expect = vec![vec!['3', '4'], vec!['1', '2']];
        assert_eq!(flip_h(&grid), expect);
    }
    #[test]
    fn flip_h_3() {
        let grid = vec![
            vec!['1', '2', '3'],
            vec!['4', '5', '6'],
            vec!['7', '8', '9'],
        ];
        let expect = vec![
            vec!['7', '8', '9'],
            vec!['4', '5', '6'],
            vec!['1', '2', '3'],
        ];
        assert_eq!(flip_h(&grid), expect);
    }

    #[test]
    fn flip_v_2() {
        let grid = vec![vec!['1', '2'], vec!['3', '4']];
        let expect = vec![vec!['2', '1'], vec!['4', '3']];
        assert_eq!(flip_v(&grid), expect);
    }
    #[test]
    fn flip_v_3() {
        let grid = vec![
            vec!['1', '2', '3'],
            vec!['4', '5', '6'],
            vec!['7', '8', '9'],
        ];
        let expect = vec![
            vec!['3', '2', '1'],
            vec!['6', '5', '4'],
            vec!['9', '8', '7'],
        ];
        assert_eq!(flip_v(&grid), expect);
    }

    #[test]
    fn divide_in_2() {
        let grid = vec![
            vec!['1', '2', '5', '6', '9', '0'],
            vec!['3', '4', '7', '8', '1', '2'],
            vec!['a', 'b', 'e', 'f', 'i', 'j'],
            vec!['c', 'd', 'g', 'h', 'k', 'l'],
            vec!['m', 'n', 'q', 'r', 'u', 'v'],
            vec!['o', 'p', 's', 't', 'w', 'x'],
        ];
        let expect = vec![
            vec![
                vec![vec!['1', '2'], vec!['3', '4']],
                vec![vec!['5', '6'], vec!['7', '8']],
                vec![vec!['9', '0'], vec!['1', '2']],
            ],
            vec![
                vec![vec!['a', 'b'], vec!['c', 'd']],
                vec![vec!['e', 'f'], vec!['g', 'h']],
                vec![vec!['i', 'j'], vec!['k', 'l']],
            ],
            vec![
                vec![vec!['m', 'n'], vec!['o', 'p']],
                vec![vec!['q', 'r'], vec!['s', 't']],
                vec![vec!['u', 'v'], vec!['w', 'x']],
            ],
        ];
        assert_eq!(divide_2(&grid), expect);
    }

    #[test]
    fn divide_in_3() {
        let grid = vec![
            vec!['1', '2', '3', 'a', 'b', 'c'],
            vec!['4', '5', '6', 'd', 'e', 'f'],
            vec!['7', '8', '9', 'g', 'h', 'i'],
            vec!['j', 'k', 'l', '1', '1', '1'],
            vec!['m', 'n', 'o', '2', '2', '2'],
            vec!['p', 'q', 'r', '3', '3', '3'],
        ];
        let expect = vec![
            vec![
                vec![
                    vec!['1', '2', '3'],
                    vec!['4', '5', '6'],
                    vec!['7', '8', '9'],
                ],
                vec![
                    vec!['a', 'b', 'c'],
                    vec!['d', 'e', 'f'],
                    vec!['g', 'h', 'i'],
                ],
            ],
            vec![
                vec![
                    vec!['j', 'k', 'l'],
                    vec!['m', 'n', 'o'],
                    vec!['p', 'q', 'r'],
                ],
                vec![
                    vec!['1', '1', '1'],
                    vec!['2', '2', '2'],
                    vec!['3', '3', '3'],
                ],
            ],
        ];
        assert_eq!(divide_3(&grid), expect);
    }

    #[test]
    fn merge_3() {
        let grid = vec![
            vec![
                vec![
                    vec!['1', '2', '3'],
                    vec!['a', 'b', 'c'],
                    vec!['9', '8', '7'],
                ],
                vec![
                    vec!['4', '5', '6'],
                    vec!['d', 'e', 'f'],
                    vec!['6', '5', '4'],
                ],
                vec![
                    vec!['7', '8', '9'],
                    vec!['g', 'h', 'i'],
                    vec!['3', '2', '1'],
                ],
            ],
            vec![
                vec![
                    vec!['m', 'n', 'o'],
                    vec!['a', 'b', 'c'],
                    vec!['1', '1', '1'],
                ],
                vec![
                    vec!['p', 'q', 'r'],
                    vec!['d', 'e', 'f'],
                    vec!['2', '2', '2'],
                ],
                vec![
                    vec!['s', 't', 'u'],
                    vec!['g', 'h', 'i'],
                    vec!['3', '3', '3'],
                ],
            ],
            vec![
                vec![
                    vec!['1', '2', '3'],
                    vec!['a', 'b', 'c'],
                    vec!['9', '8', '7'],
                ],
                vec![
                    vec!['4', '5', '6'],
                    vec!['d', 'e', 'f'],
                    vec!['6', '5', '4'],
                ],
                vec![
                    vec!['7', '8', '9'],
                    vec!['g', 'h', 'i'],
                    vec!['3', '2', '1'],
                ],
            ],
        ];

        let expect = vec![
            vec!['1', '2', '3', '4', '5', '6', '7', '8', '9'],
            vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i'],
            vec!['9', '8', '7', '6', '5', '4', '3', '2', '1'],
            vec!['m', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u'],
            vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i'],
            vec!['1', '1', '1', '2', '2', '2', '3', '3', '3'],
            vec!['1', '2', '3', '4', '5', '6', '7', '8', '9'],
            vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i'],
            vec!['9', '8', '7', '6', '5', '4', '3', '2', '1'],
        ];

        assert_eq!(merge(&grid), expect);
    }
}
