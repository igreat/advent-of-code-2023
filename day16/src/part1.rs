pub fn run(input: &str) -> usize {
    let grid = parse_input(input);
    let beam = Beam {
        x: 0,
        y: 0,
        direction: Direction::Right,
    };
    let mut visited = [
        vec![vec![false; grid[0].len()]; grid.len()],
        vec![vec![false; grid[0].len()]; grid.len()],
        vec![vec![false; grid[0].len()]; grid.len()],
        vec![vec![false; grid[0].len()]; grid.len()],
    ];
    visited[beam.direction as usize][beam.y as usize][beam.x as usize] = true;
    traverse_beam(beam, &grid, &mut visited);
    let mut count = 0;
    for y in 0..visited[0].len() {
        for x in 0..visited[0][0].len() {
            if visited[0][y][x] || visited[1][y][x] || visited[2][y][x] || visited[3][y][x] {
                count += 1;
            }
        }
    }
    count
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up = 0,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone, Copy)]
struct Beam {
    x: isize,
    y: isize,
    direction: Direction,
}

fn traverse_beam(mut beam: Beam, grid: &Vec<Vec<u8>>, visited: &mut [Vec<Vec<bool>>; 4]) {
    loop {
        let dir_idx = beam.direction as usize;
        visited[dir_idx][beam.y as usize][beam.x as usize] = true;
        match grid[beam.y as usize][beam.x as usize] {
            b'-' => match beam.direction {
                Direction::Up | Direction::Down => {
                    let split_beam = Beam {
                        x: beam.x,
                        y: beam.y,
                        direction: Direction::Right,
                    };
                    traverse_beam(split_beam, grid, visited);
                    beam.direction = Direction::Left;
                }
                _ => {}
            },
            b'|' => match beam.direction {
                Direction::Left | Direction::Right => {
                    let split_beam = Beam {
                        x: beam.x,
                        y: beam.y,
                        direction: Direction::Down,
                    };
                    traverse_beam(split_beam, grid, visited);
                    beam.direction = Direction::Up;
                }
                _ => {}
            },
            b'/' => {
                beam.direction = match beam.direction {
                    Direction::Down => Direction::Left,
                    Direction::Right => Direction::Up,
                    Direction::Up => Direction::Right,
                    Direction::Left => Direction::Down,
                }
            }
            b'\\' => {
                beam.direction = match beam.direction {
                    Direction::Down => Direction::Right,
                    Direction::Right => Direction::Down,
                    Direction::Up => Direction::Left,
                    Direction::Left => Direction::Up,
                }
            }
            _ => {}
        }
        move_beam(&mut beam);
        let dir_idx = beam.direction as usize;
        if beam.x < 0
            || beam.y < 0
            || beam.x >= grid[0].len() as isize
            || beam.y >= grid.len() as isize
            || visited[dir_idx][beam.y as usize][beam.x as usize]
        {
            break;
        }
    }
}

fn move_beam(beam: &mut Beam) {
    match beam.direction {
        Direction::Up => beam.y -= 1,
        Direction::Right => beam.x += 1,
        Direction::Down => beam.y += 1,
        Direction::Left => beam.x -= 1,
    }
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.as_bytes().to_vec()).collect()
}

fn display_visited(visited: &[Vec<Vec<bool>>; 4]) {
    // display the union of all visited cells
    let mut display = vec![vec![b'.'; visited[0][0].len()]; visited[0].len()];
    for y in 0..visited[0].len() {
        for x in 0..visited[0][0].len() {
            if visited.iter().any(|dir| dir[y][x]) {
                display[y][x] = b'#';
            }
        }
    }
    for line in display {
        println!("{}", String::from_utf8(line).unwrap());
    }
    println!();
}
