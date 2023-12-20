fn main() {
    // ### Part 1 ###
    let input = std::fs::read_to_string("day6_input.txt").unwrap();
    let mut input = input.lines();
    let times = input
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<u32>().unwrap());

    let distances = input
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<u32>().unwrap());

    let num_won_races = times
        .zip(distances)
        .map(|(time, distance)| get_num_wins(&(time as f64), &(distance as f64)));

    println!("Part 1 {}", num_won_races.product::<u32>());

    // ### Part 2 ###
    let input = std::fs::read_to_string("day6_input.txt").unwrap();
    let mut input = input.lines();
    let time = input
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1) // skip "Time:"
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    let distance = input
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1) // skip "Distance:"
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    let num_wins = get_num_wins(&(time as f64), &(distance as f64));

    println!("Part 2 {}", num_wins);
}

fn get_num_wins(t: &f64, d: &f64) -> u32 {
    let mid = t / 2.0;
    let diff = ((t * t) / 4.0 - d).sqrt();

    ((mid + diff).ceil() - (mid - diff).floor()) as u32 - 1
}
