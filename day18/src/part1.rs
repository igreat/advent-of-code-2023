pub fn run(input: &str) -> usize {
    let instructions = parse_input(input);

    let mut x = 0;
    let mut y = 0;

    let mut area = 0;

    let mut prev_x;
    let mut prev_y;
    let mut length = 0;

    for Instruction { direction, count } in instructions {
        prev_x = x;
        prev_y = y;

        match direction {
            Direction::Up => y -= count as i32,
            Direction::Down => y += count as i32,
            Direction::Left => x -= count as i32,
            Direction::Right => x += count as i32,
        }
        length += count;
        area += det(prev_x, prev_y, x, y);
    }

    ((area.abs() + length as i32) / 2) as usize + 1
}

fn det(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    x1 * y2 - y1 * x2
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    count: usize,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let direction = match &line[0..1] {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => Direction::Up,
            };
            let count = line[1..]
                .split_whitespace()
                .next()
                .unwrap()
                .parse()
                .unwrap();
            Instruction { direction, count }
        })
        .collect()
}
