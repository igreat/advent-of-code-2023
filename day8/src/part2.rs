// get gcd
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

    let num_travelers = current_nodes.len();
    let mut travel_times = vec![0; num_travelers];
    let mut num_steps = 0;
    let mut j = 0;
    while j < num_travelers {
        for m in &moves {
            num_steps += 1;
            let mut i = 0;
            while i < current_nodes.len() {
                let &(l, r) = nodes.get(current_nodes[i]).unwrap();
                match m {
                    Move::Right => {
                        current_nodes[i] = r;
                    }
                    Move::Left => {
                        current_nodes[i] = l;
                    }
                }
                if current_nodes[i][2] == b'Z' {
                    travel_times[j] = num_steps;
                    j += 1;
                    current_nodes.remove(i);
                } else {
                    i += 1;
                }
            }
        }
    }
    for i in 0..num_travelers {
        for j in i + 1..num_travelers {
            let g = gcd(travel_times[i], travel_times[j]);
            // devide the bigger number by the gcd
            if travel_times[i] > travel_times[j] {
                travel_times[i] /= g;
            } else {
                travel_times[j] /= g;
            }
        }
    }
    let shortest_path = travel_times.iter().product::<u64>();
    println!("Part 2: {}", shortest_path);
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}
