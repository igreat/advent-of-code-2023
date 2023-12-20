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
    let seeds = input
        .split("\n\n")
        .collect::<Vec<&str>>()
        .get(0)
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .filter_map(|x| x.parse::<i64>().ok());

    let seed_ranges: Vec<(i64, i64)> = seeds
        .clone()
        .collect::<Vec<i64>>()
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[1] + chunk[0]))
        .collect();

    let mut result_ranges = Vec::new();
    for range in seed_ranges {
        let mut current_ranges = vec![range];
        for map in &maps {
            current_ranges = current_ranges
                .iter()
                .flat_map(|range| get_result_ranges(map, *range))
                .collect();
        }
        result_ranges.extend(current_ranges);
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

fn get_result_ranges(map: &[(i64, i64, i64)], range: (i64, i64)) -> Vec<(i64, i64)> {
    if map.is_empty() {
        return vec![range];
    }

    let (start, end) = range;
    let mut result_ranges = Vec::new();
    let (dest_num, source_num, num_values) = map[0];
    let source_end = source_num + num_values - 1;

    // case 1: Both start and end are within the range
    if (start >= source_num && start <= source_end) && (end >= source_num && end <= source_end) {
        result_ranges.push((dest_num + start - source_num, dest_num + end - source_num));
    }
    // case 2: Start is within the range, end is after the range
    else if start >= source_num && start <= source_end && end > source_end {
        result_ranges.push((dest_num + start - source_num, dest_num + num_values - 1));
        result_ranges.extend(get_result_ranges(&map[1..], (source_end + 1, end)));
    }
    // case 3: Start is before the range, end is within the range
    else if start < source_num && (end >= source_num && end <= source_end) {
        result_ranges.extend(get_result_ranges(&map[1..], (start, source_num - 1)));
        result_ranges.push((dest_num, dest_num + end - source_num));
    }
    // case 4: Both start and end are before the range
    else if end < source_num && start < source_num {
        result_ranges.extend(get_result_ranges(&map[1..], range));
    }
    // case 5: Both start and end are after the range
    else if start > source_end && end > source_end {
        result_ranges.extend(get_result_ranges(&map[1..], range));
    }
    // case 6: Start is before the range, end is after the range
    else if start < source_num && end > source_end {
        result_ranges.extend(get_result_ranges(&map[1..], (start, source_num - 1)));
        result_ranges.push((dest_num, dest_num + num_values - 1));
        result_ranges.extend(get_result_ranges(&map[1..], (source_end + 1, end)));
    }

    result_ranges
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_result_ranges() {
        let map = vec![(49, 53, 8), (0, 11, 42), (42, 0, 7), (57, 7, 4)];
        let range = (57, 70);
        let result = get_result_ranges(&map, range);
        assert_eq!(result, vec![(53, 56), (61, 70)]);
    }
}
