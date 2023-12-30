pub fn run(input: &str) -> usize {
    let islands = parse_input(input);
    let mut result = 0;
    let mut ref_idx = Vec::new();

    for island in islands {
        ref_idx.clear();
        // HORIZONTAL CHECK
        for line in &island {
            let mut curr_ref_idx = Vec::new();
            // for &j in &prev_ref_idx {
            for j in 1..line.len() {
                let ref_len = j.min(line.len() - j);

                if are_reverse(&line[(j - ref_len)..j], &line[j..(j + ref_len)]) {
                    curr_ref_idx.push(j);
                }
            }
            ref_idx.push(curr_ref_idx);
        }

        // find the index that is common to all rows except one
        let mut common_idx = None;
        let mut common;
        'pos: for i in 1..island[0].len() {
            common = true;
            for refs in &ref_idx {
                if !refs.contains(&i) {
                    if !common {
                        // if already not common, then it's uncommon twice
                        continue 'pos;
                    }
                    common = false;
                }
            }
            if !common {
                common_idx = Some(i);
            }
        }
        if let Some(idx) = common_idx {
            result += idx;
            continue;
        }

        // VERTICAL CHECK
        ref_idx.clear();
        for j in 0..island[0].len() {
            let mut curr_ref_idx = Vec::new();
            for i in 1..island.len() {
                let ref_len = i.min(island.len() - i);

                // pluck out the two columns
                let s1: Vec<u8> = island[(i - ref_len)..i].iter().map(|row| row[j]).collect();
                let s2: Vec<u8> = island[i..(i + ref_len)].iter().map(|row| row[j]).collect();
                if are_reverse(&s1, &s2) {
                    curr_ref_idx.push(i);
                }
            }
            ref_idx.push(curr_ref_idx);
        }

        // find the index that is common to all rows except one
        let mut common_idx = None;
        let mut common;
        'pos: for i in 1..island.len() {
            common = true;
            for refs in &ref_idx {
                if !refs.contains(&i) {
                    if !common {
                        // if already not common, then it's uncommon twice
                        continue 'pos;
                    }
                    common = false;
                }
            }
            if !common {
                common_idx = Some(i);
            }
        }
        if let Some(idx) = common_idx {
            result += 100 * idx;
        }
    }
    result
}

fn are_reverse(s1: &[u8], s2: &[u8]) -> bool {
    s1.iter().zip(s2.iter().rev()).all(|(c1, c2)| c1 == c2)
}

fn parse_input(input: &str) -> Vec<Vec<&[u8]>> {
    input
        .split("\n\n")
        .map(|island| island.lines().map(|line| line.as_bytes()).collect())
        .collect()
}
