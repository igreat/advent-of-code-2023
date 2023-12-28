pub fn run(input: &str) -> usize {
    let islands = parse_input(input);
    let mut result = 0;
    let mut prev_ref_idx = Vec::new();
    let mut curr_ref_idx = Vec::new();
    let mut s1: Vec<u8>;
    let mut s2: Vec<u8>;
    for island in islands {
        prev_ref_idx.clear();
        curr_ref_idx.clear();
        // horizontal
        for line in &island {
            if prev_ref_idx.is_empty() {
                prev_ref_idx = (1..line.len()).collect();
            }
            for &j in &prev_ref_idx {
                let ref_len = j.min(line.len() - j);

                if are_reverse(&line[(j - ref_len)..j], &line[j..(j + ref_len)]) {
                    curr_ref_idx.push(j);
                }
            }
            std::mem::swap(&mut prev_ref_idx, &mut curr_ref_idx);
            if prev_ref_idx.is_empty() {
                break;
            }
            curr_ref_idx.clear();
        }

        if prev_ref_idx.len() == 1 {
            result += prev_ref_idx[0];
            continue;
        }

        // vertical
        prev_ref_idx.clear();
        curr_ref_idx.clear();
        for j in 0..island[0].len() {
            if prev_ref_idx.is_empty() {
                prev_ref_idx = (1..island.len()).collect();
            }
            for &i in &prev_ref_idx {
                let ref_len = i.min(island.len() - i);

                // pluck out the two columns
                s1 = island[(i - ref_len)..i].iter().map(|row| row[j]).collect();
                s2 = island[i..(i + ref_len)].iter().map(|row| row[j]).collect();
                if are_reverse(&s1, &s2) {
                    curr_ref_idx.push(i);
                }
            }
            std::mem::swap(&mut prev_ref_idx, &mut curr_ref_idx);
            if prev_ref_idx.is_empty() {
                break;
            }
            curr_ref_idx.clear();
        }

        if prev_ref_idx.len() == 1 {
            result += 100 * prev_ref_idx[0];
        }
    }
    result
}

fn are_reverse(s1: &[u8], s2: &[u8]) -> bool {
    s1.iter().zip(s2.iter().rev()).all(|(c1, c2)| c1 == c2)
}

fn parse_input(input: &str) -> Vec<Vec<Vec<u8>>> {
    input
        .split("\n\n")
        .map(|island| {
            island
                .lines()
                .map(|line| line.as_bytes().to_vec())
                .collect()
        })
        .collect()
}
