use std::collections::HashMap;

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
    let nodes: HashMap<&str, (&str, &str)> = lines
        .map(|line| (&line[..3], (&line[7..10], &line[12..15])))
        .collect();

    let mut current_node = "AAA";

    let mut num_steps = 0;
    'outer: loop {
        for m in &moves {
            num_steps += 1;
            let &(l, r) = nodes.get(current_node).unwrap();
            match m {
                Move::Right => current_node = r,
                Move::Left => current_node = l,
            }
            if current_node == "ZZZ" {
                break 'outer;
            }
        }
    }
    println!("Part 1: {}", num_steps);
}
