use std::collections::VecDeque;

const NUM_STEPS: usize = 65;
pub fn run(input: &str) -> usize {
    let grid = parse_input(input);
    let (x, y) = find_start(&grid);
    bfs(&grid, x, y, NUM_STEPS)
}

fn bfs(grid: &Vec<Vec<u8>>, x: usize, y: usize, steps: usize) -> usize {
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut queue = VecDeque::new();
    let mut current_distance = 0;
    queue.push_back((x, y, 0));
    while !queue.is_empty() {
        let (x, y, distance) = queue.pop_front().unwrap();
        if distance == steps || visited[y][x] {
            continue;
        }
        if distance > current_distance {
            for row in &mut visited {
                row.fill(false);
            }
            current_distance = distance;
        }

        visited[y][x] = true;
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

    visited.iter().flatten().filter(|&&cell| cell).count()
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
