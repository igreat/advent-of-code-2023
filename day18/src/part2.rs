pub fn run(input: &str) -> usize {
    let instructions = parse_input(input);

    let mut x = 0;
    let mut y = 0;

    let mut area = 0;

    let mut prev_x;
    let mut prev_y;
    let mut length = 0;

    for instruction in instructions {
        prev_x = x;
        prev_y = y;

        match instruction.direction {
            Direction::Up => y -= (instruction.count) as i64,
            Direction::Down => y += (instruction.count) as i64,
            Direction::Left => x -= (instruction.count) as i64,
            Direction::Right => x += (instruction.count) as i64,
        }
        length += instruction.count;
        area += det(prev_x, prev_y, x, y);
    }

    ((area.abs() + length as i64) / 2) as usize + 1
}

fn det(x1: i64, y1: i64, x2: i64, y2: i64) -> i64 {
    x1 * y2 - y1 * x2
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    count: u64,
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
            let line = line.split('#').nth(1).unwrap();

            let count = u64::from_str_radix(&line[..5], 16).unwrap();

            let direction = match line.as_bytes()[5] {
                b'0' => Direction::Right,
                b'1' => Direction::Down,
                b'2' => Direction::Left,
                b'3' => Direction::Up,
                _ => Direction::Up,
            };
            Instruction { direction, count }
        })
        .collect()
}
