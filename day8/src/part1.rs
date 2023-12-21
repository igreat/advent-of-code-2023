use rustc_hash::FxHashMap;

#[derive(Debug)]
enum Move {
    Right,
    Left,
}

pub fn part1() {
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
    let nodes: FxHashMap<&[u8], (&[u8], &[u8])> = lines
        .map(|line| {
            let bytes = line.as_bytes();
            (&bytes[..3], (&bytes[7..10], &bytes[12..15]))
        })
        .collect();

    let mut current_node = "AAA".as_bytes();
    let target_node = "ZZZ".as_bytes();
    let mut num_steps: usize = 0;
    'outer: loop {
        for m in &moves {
            num_steps += 1;
            let &(l, r) = nodes.get(current_node).unwrap();
            match m {
                Move::Right => current_node = r,
                Move::Left => current_node = l,
            }
            if current_node == target_node {
                break 'outer;
            }
        }
    }
    println!("Part 1: {}", num_steps);
}
