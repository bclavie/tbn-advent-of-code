fn main() {
    let inputs = include_str!("inputs/06");

    let mut lines = inputs.split('\n');
    // Part 1
    let times: Vec<i64> = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    let distances: Vec<i64> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    let ways_to_beat_record: Vec<i64> = times
        .iter()
        .zip(distances.clone())
        .map(|(time, distance)| {
            let mut ways = 0;
            for hold_duration in 1..*time {
                let distance_travelled = (time - hold_duration) * hold_duration;
                if distance_travelled > distance {
                    ways += 1;
                }
            }
            ways
        })
        .collect();

    let part1: i64 = ways_to_beat_record.iter().product();
    println!("part1: {}", part1);

    // part 2
    let time2_vec: Vec<String> = times.iter().map(|i| i.to_string()).collect();
    let time2 = time2_vec.join("").parse::<i64>().unwrap();
    let distance2_vec: Vec<String> = distances.iter().map(|i| i.to_string()).collect();
    let distance2 = distance2_vec.join("").parse::<i64>().unwrap();
    let mut ways_to_beat_record = 0;
    for hold_duration in 1..time2 {
        let distance_travelled = (time2 - hold_duration) * hold_duration;
        if distance_travelled > distance2 {
            ways_to_beat_record += 1;
        }
    }

    println!("part2: {}", ways_to_beat_record);
}
