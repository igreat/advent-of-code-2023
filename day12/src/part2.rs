pub fn run(input: &str) -> usize {
    let rows = parse_input(input);
    let mut count = 0;
    let mut prev_dp = vec![];
    let mut curr_dp = vec![];
    for row in rows {
        // let mut springs = row.state.clone();
        let (springs, damaged) = row;
        count += count_ways(&springs[..], &damaged[..], &mut prev_dp, &mut curr_dp);
    }
    count
}

fn count_ways(
    springs: &[u8],
    damaged: &[usize],
    prev_dp: &mut Vec<usize>,
    curr_dp: &mut Vec<usize>,
) -> usize {
    let (mut prev_dp, mut curr_dp) = (prev_dp, curr_dp);
    prev_dp.resize(springs.len() + 1, 0);
    curr_dp.resize(springs.len() + 1, 0);

    prev_dp.fill(0);
    prev_dp[springs.len()] = 1;
    for j in (0..springs.len()).rev().take_while(|&j| springs[j] != b'#') {
        prev_dp[j] = 1;
    }

    for &n in damaged.iter().rev() {
        let mut streak = 0;
        curr_dp[springs.len()] = 0;
        for (j, &p) in springs.iter().enumerate().rev() {
            streak = if p == b'.' { 0 } else { streak + 1 };
            curr_dp[j] = if p == b'#' { 0 } else { curr_dp[j + 1] };

            let prev_spring = springs.get(j.wrapping_sub(1));
            let next = springs.get(j + n);
            if streak >= n && prev_spring != Some(&b'#') && next != Some(&b'#') {
                curr_dp[j] += prev_dp[(j + n + 1).min(springs.len())];
            }
        }
        std::mem::swap(&mut curr_dp, &mut prev_dp);
    }

    prev_dp[0]
}

// returns Vec<(springs, damaged)>
fn parse_input(input: &str) -> Vec<(Vec<u8>, Vec<usize>)> {
    input
        .lines()
        .map(|line| {
            let (springs, damaged) = line.split_once(" ").unwrap();
            let springs = [springs; 5].join("?");
            let damaged = [damaged; 5].join(",");

            let damaged = damaged
                .split(",")
                .map(|s| s.parse::<usize>().unwrap())
                .collect();

            (springs.into_bytes(), damaged)
        })
        .collect()
}
