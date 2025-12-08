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

    // Get & sort distances --
    let mut distances: Vec<((Point, Point), isize)> = Vec::new();
    for (i, from) in boxes.iter().enumerate() {
        for to in boxes.iter().skip(i).filter(|b| *b != from) {
            let from = from.iter().next().unwrap();
            let to = to.iter().next().unwrap();

            distances.push(((*from, *to), euclidean_distance(from, to)));
        }
    }
    distances.sort_unstable_by_key(|(_, v)| *v);

    // --
    for (i, ((pt1, pt2), _)) in distances.iter().enumerate() {
        if i == nb {
            boxes.sort_unstable_by_key(|b| b.len());
            println!(
                "Part one {}",
                boxes.iter().rev().take(3).fold(1, |acc, pt| acc * pt.len())
            );
        }

        // Merge groups --
        let group1 = boxes.iter().position(|b| b.contains(pt1)).unwrap();
        let mut g1 = boxes.remove(group1);

        if let Some(group2) = boxes.iter().position(|b| b.contains(pt2)) {
            g1.extend(boxes.remove(group2));
            if boxes.is_empty() {
                println!("Part two: {}", pt1.x * pt2.x);
                break;
            }
        }
        boxes.push(g1);
    }
    Ok(())
}
