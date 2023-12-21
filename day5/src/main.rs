fn main() {
    // read input file
    let input: String = std::fs::read_to_string("day5_input.txt").unwrap();
    let mut maps: Vec<&str> = input.split("\n\n").collect();

    // ### Part 1 ###
    // will start with the seeds
    let result: Vec<i64> = maps
        .remove(0)
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .filter_map(|x| x.parse::<i64>().ok())
        .collect();

    // convert map to Vec<Vec<(i64, i64, i64)>> all at once
    let maps: Vec<Vec<(i64, i64, i64)>> = maps
        .iter()
        .map(|map| {
            map.split_once(":\n")
                .unwrap()
                .1
                .lines()
                .map(|line| {
                    let numbers = line
                        .split_whitespace()
                        .filter_map(|x| x.parse::<i64>().ok())
                        .collect::<Vec<i64>>();

                    (numbers[0], numbers[1], numbers[2])
                })
                .collect()
        })
        .collect();
    let min_part1 = get_min_location(&maps, result);

    // ### Part 2 ###
    let mut lines = input.lines();
    let seed_ranges: Vec<(i64, i64)> = lines.next().unwrap()[7..]
        .split_whitespace()
        .filter_map(|x| x.parse::<i64>().ok())
        .collect::<Vec<i64>>()
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[1] + chunk[0]))
        .collect();

    let mut result_ranges = seed_ranges;
    for map in &maps {
        result_ranges = get_result_ranges(map, result_ranges);
    }

    let min_part2 = result_ranges.iter().map(|(start, _)| *start).min().unwrap();

    println!("Part 1: {}", min_part1);
    println!("Part 2: {}", min_part2);
}

fn get_min_location(maps: &Vec<Vec<(i64, i64, i64)>>, mut result: Vec<i64>) -> i64 {
    for map in maps {
        // find which of the maps within map has the correct range
        for i in 0..result.len() {
            for &(dest_num, source_num, num_values) in map {
                if result[i] >= source_num && result[i] < (source_num + num_values) {
                    result[i] = dest_num + result[i] - source_num;
                    break;
                }
            }
        }
    }
    *result.iter().min().unwrap()
}

fn get_result_ranges(map: &[(i64, i64, i64)], ranges: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let mut remaining_ranges = ranges;
    let mut result_ranges = Vec::new();
    let mut new_remaining_ranges = Vec::new();

    for &(dest_num, source_num, num_values) in map {
        let source_end = source_num + num_values - 1;

        for &range in &remaining_ranges {
            let (start, end) = range;

            let start_in_range = start >= source_num && start <= source_end;
            let end_in_range = end >= source_num && end <= source_end;

            match (start_in_range, end_in_range) {
                (true, true) => {
                    result_ranges
                        .push((dest_num + start - source_num, dest_num + end - source_num));
                }
                (true, false) => {
                    result_ranges.push((dest_num + start - source_num, dest_num + num_values - 1));
                    new_remaining_ranges.push((source_end + 1, end));
                }
                (false, true) => {
                    new_remaining_ranges.push((start, source_num - 1));
                    result_ranges.push((dest_num, dest_num + end - source_num));
                }
                _ => {
                    // both completely before or after
                    if end < source_num || start > source_end {
                        new_remaining_ranges.push(range);
                    } else {
                        // encapsulates the whole valid range inside
                        // start < source_num && end > source_end
                        new_remaining_ranges.push((start, source_num - 1));
                        result_ranges.push((dest_num, dest_num + num_values - 1));
                        new_remaining_ranges.push((source_end + 1, end));
                    }
                }
            }
        }

        std::mem::swap(&mut remaining_ranges, &mut new_remaining_ranges);
        new_remaining_ranges.clear();
    }

    result_ranges.extend(remaining_ranges);

    result_ranges
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_result_ranges() {
        let map = vec![(49, 53, 8), (0, 11, 42), (42, 0, 7), (57, 7, 4)];
        let ranges = vec![(57, 70)];
        let result = get_result_ranges(&map, ranges);
        assert_eq!(result, vec![(53, 56), (61, 70)]);
    }
}
