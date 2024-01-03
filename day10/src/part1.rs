pub fn run(input: &str) -> u32 {
    let grid = parse(input);
    let (base_x, base_y) = find_start(&grid);

    // traversing the pipe until we back to the start
    let mut x = base_x;
    let mut y = base_y;
    let mut current_direction = Direction::Right;
    let mut pipe_len = 1;
    loop {
        current_direction = match current_direction.get_next(grid[y][x]) {
            Some(direction) => direction,
            None => Direction::Right,
        };

        match current_direction {
            Direction::Up | Direction::RightUp | Direction::LeftUp => y -= 1,
            Direction::Right | Direction::UpRight | Direction::DownRight => x += 1,
            Direction::Down | Direction::RightDown | Direction::LeftDown => y += 1,
            Direction::Left | Direction::UpLeft | Direction::DownLeft => x -= 1,
        }

        pipe_len += 1;
        if (x, y) == (base_x, base_y) {
            break;
        }
    }
    pipe_len / 2 // pipe_len is always even
}

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
    UpRight,
    DownRight,
    DownLeft,
    UpLeft,
    RightUp,
    RightDown,
    LeftDown,
    LeftUp,
}

impl Direction {
    fn get_next(&self, cell: u8) -> Option<Direction> {
        match cell {
            b'-' => match self {
                Direction::Right => Some(Direction::Right),
                Direction::Left => Some(Direction::Left),
                Direction::UpRight => Some(Direction::Right),
                Direction::DownRight => Some(Direction::Right),
                Direction::DownLeft => Some(Direction::Left),
                Direction::UpLeft => Some(Direction::Left),
                _ => None,
            },
            b'|' => match self {
                Direction::Up => Some(Direction::Up),
                Direction::Down => Some(Direction::Down),
                Direction::RightUp => Some(Direction::Up),
                Direction::RightDown => Some(Direction::Down),
                Direction::LeftDown => Some(Direction::Down),
                Direction::LeftUp => Some(Direction::Up),
                _ => None,
            },
            b'F' => match self {
                Direction::Left | Direction::DownLeft | Direction::UpLeft => {
                    Some(Direction::LeftDown)
                }
                Direction::Up | Direction::RightUp | Direction::LeftUp => Some(Direction::UpRight),
                _ => None,
            },
            b'L' => match self {
                Direction::Left | Direction::UpLeft | Direction::DownLeft => {
                    Some(Direction::LeftUp)
                }
                Direction::Down | Direction::RightDown | Direction::LeftDown => {
                    Some(Direction::DownRight)
                }
                _ => None,
            },
            b'7' => match self {
                Direction::Right | Direction::DownRight | Direction::UpRight => {
                    Some(Direction::RightDown)
                }
                Direction::Up | Direction::LeftUp | Direction::RightUp => Some(Direction::UpLeft),
                _ => Some(Direction::UpRight),
            },
            b'J' => match self {
                Direction::Right | Direction::UpRight | Direction::DownRight => {
                    Some(Direction::RightUp)
                }
                Direction::Down | Direction::LeftDown | Direction::RightDown => {
                    Some(Direction::DownLeft)
                }
                _ => None,
            },
            _ => None,
        }
    }
}

fn parse(input: &str) -> Vec<&[u8]> {
    input.lines().map(|line| line.as_bytes()).collect()
}

fn find_start(grid: &[&[u8]]) -> (usize, usize) {
    for (y, line) in grid.iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            if c == b'S' {
                return (x, y);
            }
        }
    }
    (0, 0)
}
