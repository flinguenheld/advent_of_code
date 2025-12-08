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
    let path = "input.txt";
    // let path = "input_example.txt";

    let mut boxes: Vec<HashSet<Point>> = std::fs::read_to_string(path)?
        .lines()
        .map(|line| {
            let mut fields = line.split(',');

            HashSet::from_iter(vec![Point {
                x: fields.next().unwrap().parse::<i64>().unwrap(),
                y: fields.next().unwrap().parse::<i64>().unwrap(),
                z: fields.next().unwrap().parse::<i64>().unwrap(),
            }])
        })
        .collect();

    // println!("{:?}", boxes);

    let mut distances: HashMap<(Point, Point), isize> = HashMap::new();

    // Get distances
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

    // loop {
    // println!("Shortest: {:?}",);
    for i in 0..1000 {
        if let Some(((pt1, pt2), value)) = distances.iter_mut().min_by_key(|(_, v)| **v) {
            // if *value == isize::MAX {
            //     break;
            // }
            *value = isize::MAX;
            // println!("deal with these points {:?} & {:?}", pt1, pt2);

            // Merge groups
            // Find both sides
            let group1 = boxes.iter().position(|b| b.contains(pt1)).unwrap();
            let g1 = boxes.remove(group1);
            if let Some(group2) = boxes.iter().position(|b| b.contains(pt2)) {
                let g2 = boxes.remove(group2);

                boxes.push(g1.union(&g2).cloned().collect());
            } else {
                boxes.push(g1);
            }
        }
    }

    // println!("{:?}", distances.iter().find(|blah| blah.1 == &0));
    for b in boxes.iter() {
        println!("{:?}", b);
    }

    // println!(
    //     "Part one: {}",
    //     groups.iter().fold(0, |acc, g| acc + g.len())
    // );

    let mut total = 1;
    for _ in 0..3 {
        let longest = boxes.iter().max_by_key(|b| b.len()).unwrap().len();
        total *= longest;
        boxes.remove(boxes.iter().position(|b| b.len() == longest).unwrap());
    }

    println!("Part one {}", total);

    Ok(())
}

// 8 -> NO :|
