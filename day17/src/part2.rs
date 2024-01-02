use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub fn run(input: &str) -> usize {
    let mut grid = parse_input(input);
    grid[0][0] = 0;
    min_heat(&grid)
}

#[derive(PartialEq, Eq)]
struct State {
    x: usize,
    y: usize,
    heat: usize,
    horizontal: bool,
}

// min-heap
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.heat.cmp(&self.heat)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

enum Direction {
    Down,
    Right,
    Up,
    Left,
}

fn min_heat(grid: &Vec<Vec<usize>>) -> usize {
    let mut heap = BinaryHeap::new();
    let mut min_heats = vec![vec![vec![usize::MAX; grid[0].len()]; grid.len()]; 2];

    min_heats[0][0][0] = 0;
    min_heats[1][0][0] = 0;

    let mut direction_queue = vec![Direction::Down];

    heap.push(State {
        x: 0,
        y: 0,
        heat: 0,
        horizontal: true,
    });
    heap.push(State {
        x: 0,
        y: 0,
        heat: 0,
        horizontal: false,
    });

    while let Some(State {
        x,
        y,
        heat,
        horizontal,
    }) = heap.pop()
    {
        if (x, y) == (grid[0].len() - 1, grid.len() - 1) {
            return heat;
        }

        if heat > min_heats[horizontal as usize][y][x] {
            continue;
        }

        if horizontal {
            direction_queue.push(Direction::Left);
            direction_queue.push(Direction::Right);
        } else {
            direction_queue.push(Direction::Up);
            direction_queue.push(Direction::Down);
        }

        'direction: for direction in &direction_queue {
            let mut next_x = x;
            let mut next_y = y;
            let mut next_heat = heat;
            // must move a minimum of 4 steps (3 + (1..7))
            for _ in 0..3 {
                match direction {
                    Direction::Up => next_y -= 1,
                    Direction::Down => next_y += 1,
                    Direction::Left => next_x -= 1,
                    Direction::Right => next_x += 1,
                }
                if next_x >= grid[0].len() || next_y >= grid.len() {
                    continue 'direction;
                }
                next_heat += grid[next_y][next_x];
            }

            for _ in 0..7 {
                match direction {
                    Direction::Up => next_y -= 1,
                    Direction::Down => next_y += 1,
                    Direction::Left => next_x -= 1,
                    Direction::Right => next_x += 1,
                }
                if next_x >= grid[0].len() || next_y >= grid.len() {
                    break;
                }

                next_heat += grid[next_y][next_x];

                let next_horizontal = !matches!(direction, Direction::Left | Direction::Right);
                if next_heat < min_heats[next_horizontal as usize][next_y][next_x] {
                    heap.push(State {
                        x: next_x,
                        y: next_y,
                        heat: next_heat,
                        horizontal: next_horizontal,
                    });
                    min_heats[next_horizontal as usize][next_y][next_x] = next_heat;
                }
            }
        }
        direction_queue.clear();
    }
    0
}

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.bytes()
                .into_iter()
                .map(|b| (b - b'0') as usize)
                .collect()
        })
        .collect()
}
