use std::collections::{HashMap, HashSet};

fn main() {
    // read input file
    let input = std::fs::read_to_string("day3_input.txt").unwrap();
    let lines: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();
    let line_len = lines[0].len();

    // --- PART 1 ---
    let mut is_part_num;
    let mut part_num_sum = 0;
    let mut current_num = "".to_string();
    for j in 0..lines.len() {
        is_part_num = false;
        for i in 0..line_len {
            if (lines[j][i] as char).is_numeric() {
                current_num.push(lines[j][i] as char);

                // check surroundings
                if !is_part_num && check_part_num(&lines, i, j) {
                    is_part_num = true;
                }
            } else if current_num.len() > 0 {
                if is_part_num {
                    part_num_sum += current_num.parse::<u32>().unwrap();
                }
                current_num = "".to_string();
                is_part_num = false;
            }
        }

        // check last number
        if current_num.len() > 0 {
            if is_part_num {
                part_num_sum += current_num.parse::<u32>().unwrap();
            }
            current_num = "".to_string();
        }
    }

    // --- PART 2 ---
    let mut gear_numbers = HashMap::new();
    let mut current_num = "".to_string();
    let mut current_locations;
    for j in 0..lines.len() {
        current_locations = HashSet::new();
        for i in 0..line_len {
            if (lines[j][i] as char).is_numeric() {
                current_num.push(lines[j][i] as char);

                // check surroundings
                let locations = get_symbol_locations(&lines, i, j);
                if locations.len() > 0 {
                    current_locations.extend(locations);
                }
            } else if current_num.len() > 0 {
                if current_locations.len() > 0 {
                    for (x, y) in current_locations {
                        let mut current_vec =
                            gear_numbers.get(&(x, y)).unwrap_or(&Vec::new()).clone();
                        current_vec.push(current_num.parse::<u32>().unwrap());
                        gear_numbers.insert((x, y), current_vec);
                    }
                }
                current_num = "".to_string();
                current_locations = HashSet::new();
            }
        }

        // check last number
        if current_num.len() > 0 {
            if current_locations.len() > 0 {
                for (x, y) in current_locations {
                    let mut current_vec = gear_numbers.get(&(x, y)).unwrap_or(&Vec::new()).clone();
                    current_vec.push(current_num.parse::<u32>().unwrap());
                    gear_numbers.insert((x, y), current_vec);
                }
            }
        }
    }

    let sum_gear_ratios = gear_numbers
        .values()
        .filter_map(|v| match **v {
            [a, b] => Some(a * b),
            _ => None,
        })
        .sum::<u32>();

    println!("Part 1: {}", part_num_sum);
    println!("Part 2: {}", sum_gear_ratios);
}

fn is_symbol(c: char) -> bool {
    !(c.is_alphanumeric() || (c == '.'))
}

fn check_part_num(lines: &Vec<&[u8]>, i: usize, j: usize) -> bool {
    let min_x = if i == 0 { 0 } else { i - 1 };
    let max_x = if i == lines[0].len() - 1 {
        lines[0].len() - 1
    } else {
        i + 1
    };
    let min_y = if j == 0 { 0 } else { j - 1 };
    let max_y = if j == lines.len() - 1 {
        lines.len() - 1
    } else {
        j + 1
    };

    (min_x..=max_x)
        .any(|x| (min_y..=max_y).any(|y| !(x == i && y == j) && is_symbol(lines[y][x] as char)))
}

fn get_symbol_locations(lines: &Vec<&[u8]>, i: usize, j: usize) -> Vec<(usize, usize)> {
    // will do essentially the same as check_part_num, but will return a vector of locations instead of a bool
    let min_x = if i == 0 { 0 } else { i - 1 };
    let max_x = if i == lines[0].len() - 1 {
        lines[0].len() - 1
    } else {
        i + 1
    };
    let min_y = if j == 0 { 0 } else { j - 1 };
    let max_y = if j == lines.len() - 1 {
        lines.len() - 1
    } else {
        j + 1
    };

    let mut locations = Vec::new();
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if !(x == i && y == j) && is_symbol(lines[y][x] as char) {
                locations.push((x, y));
            }
        }
    }

    locations
}
