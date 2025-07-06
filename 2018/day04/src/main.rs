use std::collections::HashMap;

fn main() {
    let path = "input.txt";
    // let path = "input_example.txt";

    let input = std::fs::read_to_string(path).unwrap();

    let mut guards: HashMap<u32, Vec<(u32, u32)>> = HashMap::new();

    for line in input.lines().filter(|l| l.contains("Guard")) {
        // Get the guard ID
        let (date, id) = line.split_once('#').unwrap();
        let id = id
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<u32>()
            .unwrap();

        // Get all his working days
        let (date, _) = if date.contains("23:") {
            // Take the day and add one -_-'
            // AOC is so vicious
            (next_day(date), 0)
        } else {
            (date[..12].to_string(), date[15..17].parse::<u32>().unwrap())
        };

        let mut this_day: Vec<(u32, &str)> = input
            .lines()
            .filter(|l| l.starts_with(&date) && !l.contains("Guard"))
            .map(|txt| {
                let (hour, action) = txt.trim_start_matches(&date).split_once("] ").unwrap();
                let hour = hour[3..].parse::<u32>().unwrap();
                (hour, action)
            })
            .collect();

        // Sort & save
        this_day.sort_by_key(|(h, _)| *h);

        let ranges_for_this_day: Vec<(u32, u32)> = this_day
            .windows(2)
            .step_by(2)
            .map(|win| (win[0].0, win[1].0))
            .collect();

        // We finally don't care of a lot of information -_-'
        // let month = date[6..8].parse::<u32>().unwrap();
        // let day = date[9..11].parse::<u32>().unwrap();

        guards
            .entry(id)
            .and_modify(|e| e.extend(ranges_for_this_day.iter()))
            .or_insert(ranges_for_this_day);
    }

    // Who's the biggest sleeper ?
    if let Some((most_aslept_id, most_aslept_times)) = guards
        .iter()
        .max_by(|(_, a), (_, b)| time_asleep(a).cmp(&time_asleep(b)))
    {
        let mut minute_counters: HashMap<u32, u32> = HashMap::new();

        // When ?
        for (start, end) in most_aslept_times.iter() {
            for i in *start..*end {
                minute_counters
                    .entry(i)
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }
        }

        let maxi = minute_counters.iter().max_by_key(|(_, v)| *v).unwrap();
        println!("Part one: {}", most_aslept_id * maxi.0);
    } else {
        println!("Not found");
    }

    // --
    // For all guards, sum all of his minutes
    let minutes_per_guard: HashMap<u32, HashMap<u32, u32>> = guards
        .iter()
        .map(|(id, ranges)| {
            let mut minute_counters: HashMap<u32, u32> = HashMap::new();

            for (from, to) in ranges.iter() {
                for i in *from..*to {
                    minute_counters
                        .entry(i)
                        .and_modify(|e| *e += 1)
                        .or_insert(1);
                }
            }
            (*id, minute_counters)
        })
        .collect();

    let minutes_per_guard: HashMap<u32, (u32, u32)> = minutes_per_guard
        .iter()
        .map(|(id, range)| {
            // Guard #683 doesn't sleep so he doesn't have any range -_-'
            // AOC is soo vicious
            let maxi = range.iter().max_by_key(|b| b.1).unwrap_or((&0, &0));

            (*id, (*maxi.0, *maxi.1))
        })
        .collect();

    if let Some(maxi) = minutes_per_guard.iter().max_by_key(|g| g.1 .1) {
        println!("Part two: {}", maxi.0 * maxi.1 .0);
    }
}

fn time_asleep(guard: &[(u32, u32)]) -> u32 {
    guard.iter().fold(0, |acc, t| acc + t.1 - t.0)
}
fn next_day(date: &str) -> String {
    let max_30 = [4, 6, 9, 11];

    let mut month = date[6..8].parse::<u8>().unwrap();
    let mut day = date[9..11].parse::<u8>().unwrap() + 1;

    if (month == 2 && day > 28) || (max_30.contains(&month) && day > 30) || day > 31 {
        day = 0;
        month += 1;
    }

    format!("{}-{:02}-{:02} ", &date[..5], month, day)
}
