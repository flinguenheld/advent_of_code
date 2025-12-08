use std::collections::{HashMap, HashSet};

use anyhow::Result;

#[derive(Debug, Eq, PartialEq, PartialOrd, Hash, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

fn euclidean_distance(p1: &Point, p2: &Point) -> isize {
    let aaa = (p1.x - p2.x).pow(2) + (p1.y - p2.y).pow(2) + (p1.z - p2.z).pow(2);
    (aaa as f64).sqrt() as isize
}

fn main() -> Result<()> {
    let (path, nb) = ("input.txt", 1000);
    // let (path, nb) = ("input_example.txt", 10);

    let mut boxes: Vec<Vec<Point>> = std::fs::read_to_string(path)?
        .lines()
        .map(|line| {
            let mut fields = line.split(',');

            vec![Point {
                x: fields.next().unwrap().parse::<i64>().unwrap(),
                y: fields.next().unwrap().parse::<i64>().unwrap(),
                z: fields.next().unwrap().parse::<i64>().unwrap(),
            }]
        })
        .collect();

    // Get distances --
    let mut distances: HashMap<(Point, Point), isize> = HashMap::new();
    for from in boxes.iter() {
        for to in boxes.iter() {
            let from = from.iter().next().unwrap();
            let to = to.iter().next().unwrap();

            if to != from {
                let d = euclidean_distance(from, to);
                if !distances.contains_key(&(*to, *from)) {
                    distances.insert((*from, *to), d);
                }
            }
        }
    }

    let mut i = 0;
    loop {
        if let Some(((pt1, pt2), value)) = distances
            .iter_mut()
            .filter(|(_, v)| **v > 0)
            .min_by_key(|(_, v)| **v)
        {
            if i == nb {
                let mut to_add = Vec::new();
                while to_add.len() < 3 {
                    to_add.push(
                        boxes
                            .iter()
                            .filter(|b| !to_add.contains(*b))
                            .max_by_key(|b| b.len())
                            .unwrap()
                            .clone(),
                    );
                }

                println!(
                    "Part one {}",
                    to_add.iter().fold(1, |acc, (pt)| acc * pt.len())
                );
                break;
            }
            *value = 0;

            // Merge groups --
            let group1 = boxes.iter().position(|b| b.contains(pt1)).unwrap();
            let mut g1 = boxes.remove(group1);
            if let Some(group2) = boxes.iter().position(|b| b.contains(pt2)) {
                g1.extend(boxes.remove(group2));
            }
            boxes.push(g1);
        }
        i += 1;
    }

    // println!("{:?}", distances.iter().find(|blah| blah.1 == &0));
    // for b in boxes.iter() {
    //     println!("{:?}", b);
    // }

    // println!(
    //     "Part one: {}",
    //     groups.iter().fold(0, |acc, g| acc + g.len())
    // );

    Ok(())
}

// 8 -> NO :|
