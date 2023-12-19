fn main() {
    // read input file
    let input: String = std::fs::read_to_string("day5_input.txt").unwrap();
    let mut maps: Vec<&str> = input.split("\n\n").collect();

    // will start with the seeds
    let mut result: Vec<i64> = maps
        .remove(0)
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .filter_map(|x| x.parse::<i64>().ok())
        .collect();

    for map in maps {
        let map: Vec<(i64, i64, i64)> = map
            .split_once(":\n")
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
            .collect();

        // find which of the maps within map has the correct range
        'sources: for i in 0..result.len() {
            for (dest_num, source_num, num_values) in &map {
                if result[i] >= *source_num && result[i] < (*source_num + *num_values) {
                    result[i] = *dest_num + result[i] - *source_num;
                    continue 'sources;
                }
            }
        }
    }

    println!("Part 1: {}", result.iter().min().unwrap());
}
