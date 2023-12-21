use std::collections::HashMap;

#[derive(Debug)]
enum Move {
    Right,
    Left,
}

pub fn part2() {
    let input = std::fs::read_to_string("day8_input.txt").unwrap();
    let mut lines = input.lines();
    let moves: Vec<Move> = lines
        .next()
        .unwrap()
        .as_bytes()
        .iter()
        .filter_map(|c| match c {
            b'R' => Some(Move::Right),
            b'L' => Some(Move::Left),
            _ => None,
        })
        .collect();

    lines.next();
    let mut current_nodes = Vec::new();
    let mut nodes = HashMap::new();
    for line in lines {
        let bytes = line.as_bytes();
        if &bytes[2] == &b'A' {
            current_nodes.push(&bytes[..3]);
        }
        nodes.insert(&bytes[..3], (&bytes[7..10], &bytes[12..15]));
    }
    let mut num_steps = 0;
    let mut all_zs;
    'outer: loop {
        for m in &moves {
            all_zs = true;
            num_steps += 1;
            for i in 0..current_nodes.len() {
                let &(l, r) = nodes.get(current_nodes[i]).unwrap();
                match m {
                    Move::Right => {
                        current_nodes[i] = r;
                    }
                    Move::Left => {
                        current_nodes[i] = l;
                    }
                }
                if current_nodes[i][2] != b'Z' {
                    all_zs = false;
                }
            }
            if all_zs {
                break 'outer;
            }
        }
    }
    println!("Part 2: {}", num_steps);
}
