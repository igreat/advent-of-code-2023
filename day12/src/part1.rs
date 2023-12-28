pub fn run(input: &str) -> usize {
    let rows = parse_input(input);
    let mut count = 0;
    for row in rows {
        // let mut springs = row.state.clone();
        let (mut springs, question_marks, damaged) = row;
        count += num_possible_springs(&mut springs, &damaged, &question_marks);
    }
    count
}

fn num_possible_springs(
    springs: &mut Vec<u8>,
    damaged: &Vec<u32>,
    question_marks: &[usize],
) -> usize {
    // base case
    if question_marks.len() == 0 {
        let is_valid = is_valid_row(springs, damaged);
        return is_valid as usize;
    }

    // prune branches
    if !possible_branch(springs, damaged) {
        return 0;
    }

    let mut count = 0;
    let i = question_marks[0];
    springs[i] = b'.';
    count += num_possible_springs(&mut springs.clone(), damaged, &question_marks[1..]);
    springs[i] = b'#';
    count += num_possible_springs(springs, damaged, &question_marks[1..]);

    count
}

fn is_valid_row(springs: &Vec<u8>, damaged: &Vec<u32>) -> bool {
    // check if the row is valid
    let mut last_damaged = 0;
    let mut j = 0;
    for &spring in springs {
        if spring == b'.' {
            if last_damaged == 0 {
                continue;
            } else if damaged[j] == last_damaged {
                last_damaged = 0;
                j += 1;
            } else {
                return false;
            }
        }
        if spring == b'#' {
            if j == damaged.len() {
                return false;
            }
            last_damaged += 1;
        }
    }

    // check for last damaged
    if last_damaged != 0 {
        if j + 1 != damaged.len() || damaged[j] != last_damaged {
            return false;
        } else {
            return true;
        }
    }
    if j != damaged.len() {
        return false;
    }
    true
}

fn possible_branch(springs: &Vec<u8>, damaged: &Vec<u32>) -> bool {
    let mut last_damaged = 0;
    let mut j = 0;
    for &spring in springs {
        if spring == b'?' {
            return true;
        }
        if spring == b'.' {
            if last_damaged == 0 {
                continue;
            } else if damaged[j] == last_damaged {
                last_damaged = 0;
                j += 1;
            } else {
                return false;
            }
        }
        if spring == b'#' {
            if j == damaged.len() {
                return false;
            }
            last_damaged += 1;
        }
    }

    // check for last damaged
    if last_damaged != 0 && last_damaged >= damaged[j] {
        return false;
    }

    true
}

fn parse_input(input: &str) -> Vec<(Vec<u8>, Vec<usize>, Vec<u32>)> {
    input
        .lines()
        .map(|line| {
            let (springs, damaged) = line.split_once(" ").unwrap();
            let damaged = damaged
                .split(",")
                .map(|s| s.parse::<u32>().unwrap())
                .collect();

            // get positions of all the question marks
            let mut question_marks = vec![];
            for (i, c) in springs.as_bytes().iter().enumerate() {
                if *c == b'?' {
                    question_marks.push(i);
                }
            }

            (springs.as_bytes().to_vec(), question_marks, damaged)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_spring1() {
        let input = ".###.##.#... 3,2,1";
        let (springs, _, damaged) = parse_input(input)[0].clone();
        assert!(is_valid_row(&springs, &damaged));
    }

    #[test]
    fn test_is_valid_spring2() {
        let input = ".###.##....# 3,2,1";
        let (springs, _, damaged) = parse_input(input)[0].clone();
        assert!(is_valid_row(&springs, &damaged));
    }
}
