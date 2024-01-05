use std::collections::VecDeque;

const NUM_STEPS: usize = 65;
pub fn run(input: &str) -> usize {
    let grid = parse_input(input);
    let mut visited = vec![vec![vec![false; grid[0].len()]; grid.len()]; NUM_STEPS];
    let (x, y) = find_start(&grid);
    bfs(&grid, &mut visited, x, y, NUM_STEPS)
}

fn bfs(
    grid: &Vec<Vec<u8>>,
    visited: &mut Vec<Vec<Vec<bool>>>,
    x: usize,
    y: usize,
    steps: usize,
) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back((x, y, 0));
    while !queue.is_empty() {
        let (x, y, distance) = queue.pop_front().unwrap();
        if distance >= steps || visited[distance][y][x] {
            continue;
        }
        visited[distance][y][x] = true;
        for (dx, dy) in &[(0, -1), (1, 0), (0, 1), (-1, 0)] {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx < 0 || nx >= grid[0].len() as i32 || ny < 0 || ny >= grid.len() as i32 {
                continue;
            }
            let nx = nx as usize;
            let ny = ny as usize;
            if grid[ny][nx] != b'#' {
                queue.push_back((nx, ny, distance + 1));
            }
        }
    }

    visited[steps - 1]
        .iter()
        .flatten()
        .filter(|&&cell| cell)
        .count()
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.as_bytes().to_vec()).collect()
}

fn find_start(grid: &Vec<Vec<u8>>) -> (usize, usize) {
    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == b'S' {
                return (x, y);
            }
        }
    }
    panic!("No start found");
}
