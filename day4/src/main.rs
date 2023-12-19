use std::collections::HashSet;

fn main() {
    // read input file
    let input = std::fs::read_to_string("day4_input.txt").unwrap();
    let lines = input.lines();
    let mut matches_per_card = Vec::new();

    // --- PART 1 ---
    let mut total_points = 0;
    for line in lines {
        let line = line.split_once(':').unwrap().1;
        let (card_nums, my_nums) = line.split_once('|').unwrap();
        let my_nums: HashSet<u32> = my_nums
            .split_whitespace()
            .filter_map(|x| x.parse::<u32>().ok())
            .collect();

        let card_nums: HashSet<u32> = card_nums
            .split_whitespace()
            .filter_map(|x| x.parse::<u32>().ok())
            .collect();

        let num_matches = card_nums.intersection(&my_nums).count() as u32;
        matches_per_card.push(num_matches);
        if num_matches != 0 {
            total_points += (2u32).pow(num_matches - 1);
        }
    }

    // --- PART 2 ---
    let mut total_copies = 0;
    // num of copies of each card
    let mut copies_per_card = vec![1; matches_per_card.len()];
    for (i, &num_matches) in matches_per_card.iter().enumerate() {
        total_copies += copies_per_card[i];
        for j in (i + 1)..=(i + num_matches as usize) {
            copies_per_card[j] += copies_per_card[i];
        }
    }

    println!("Part 1: {}", total_points);
    println!("Part 2: {}", total_copies);
}
