use colored::Colorize;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Cell {
    Path,
    Forest,
    LeftSlope,
    RightSlope,
    UpSlope,
    DownSlope,
    Visited,
}

pub fn run(input: &str) -> u32 {
    let mut grid = parse_input(input);
    grid[0][1] = Cell::Visited;
    traverse(&mut grid, (1, 0), 0)
}

fn traverse(grid: &mut Vec<Vec<Cell>>, start: (isize, isize), prev_length: u32) -> u32 {
    let mut current;
    let mut temp_current;
    let mut length = prev_length;
    let mut valid_moves = Vec::new();
    let mut grid_copy;
    let mut current_copy;
    let mut length_copy;

    current = start;
    loop {
        // check for end here as well
        if current.0 == grid[0].len() as isize - 2 && current.1 == grid.len() as isize - 1 {
            print_grid(&grid);
            return length;
        }

        for direction in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            temp_current = current;

            temp_current.0 += direction.0;
            temp_current.1 += direction.1;

            if temp_current.0 < 0
                || temp_current.1 < 0
                || temp_current.0 >= grid[0].len() as isize
                || temp_current.1 >= grid.len() as isize
            {
                continue;
            }

            match grid[temp_current.1 as usize][temp_current.0 as usize] {
                Cell::Path => {
                    valid_moves.push(direction);
                }
                Cell::LeftSlope => {
                    if temp_current.0 - 1 < 0
                        || temp_current.1 < 0
                        || temp_current.0 - 1 >= grid[0].len() as isize
                        || temp_current.1 >= grid.len() as isize
                        || matches!(
                            grid[temp_current.1 as usize][(temp_current.0 - 1) as usize],
                            Cell::Forest | Cell::Visited
                        )
                    {
                        continue;
                    }

                    valid_moves.push((direction.0 - 1, direction.1));
                }
                Cell::RightSlope => {
                    if temp_current.0 + 1 < 0
                        || temp_current.1 < 0
                        || temp_current.0 + 1 >= grid[0].len() as isize
                        || temp_current.1 >= grid.len() as isize
                        || matches!(
                            grid[temp_current.1 as usize][(temp_current.0 + 1) as usize],
                            Cell::Forest | Cell::Visited
                        )
                    {
                        continue;
                    }
                    valid_moves.push((direction.0 + 1, direction.1));
                }
                Cell::UpSlope => {
                    if temp_current.0 < 0
                        || temp_current.1 - 1 < 0
                        || temp_current.0 >= grid[0].len() as isize
                        || temp_current.1 - 1 >= grid.len() as isize
                        || matches!(
                            grid[(temp_current.1 - 1) as usize][temp_current.0 as usize],
                            Cell::Forest | Cell::Visited
                        )
                    {
                        continue;
                    }
                    valid_moves.push((direction.0, direction.1 - 1));
                }
                Cell::DownSlope => {
                    if temp_current.0 < 0
                        || temp_current.1 + 1 < 0
                        || temp_current.0 >= grid[0].len() as isize
                        || temp_current.1 + 1 >= grid.len() as isize
                        || matches!(
                            grid[(temp_current.1 + 1) as usize][temp_current.0 as usize],
                            Cell::Forest | Cell::Visited
                        )
                    {
                        continue;
                    }
                    valid_moves.push((direction.0, direction.1 + 1));
                }
                Cell::Forest | Cell::Visited => continue,
            }
        }

        if valid_moves.len() == 0 {
            return 0;
        }

        if valid_moves.len() == 1 {
            let direction = valid_moves[0];
            for _ in 0..direction.0.abs() {
                current.0 += direction.0.signum();
                length += 1;
                grid[current.1 as usize][current.0 as usize] = Cell::Visited;
            }
            for _ in 0..direction.1.abs() {
                current.1 += direction.1.signum();
                length += 1;
                grid[current.1 as usize][current.0 as usize] = Cell::Visited;
            }
        } else {
            let mut max_length = 0;
            for direction in valid_moves {
                grid_copy = grid.clone();
                current_copy = current;
                length_copy = length;

                for _ in 0..direction.0.abs() {
                    current_copy.0 += direction.0.signum();
                    length_copy += 1;
                    grid_copy[current_copy.1 as usize][current_copy.0 as usize] = Cell::Visited;
                }
                for _ in 0..direction.1.abs() {
                    current_copy.1 += direction.1.signum();
                    length_copy += 1;
                    grid_copy[current_copy.1 as usize][current_copy.0 as usize] = Cell::Visited;
                }

                let path_len = traverse(&mut grid_copy, current_copy, length_copy);
                if path_len > max_length {
                    max_length = path_len;
                }
            }
            return max_length;
        }

        valid_moves.clear();
    }
}

fn parse_input(input: &str) -> Vec<Vec<Cell>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Cell::Path,
                    '#' => Cell::Forest,
                    '<' => Cell::LeftSlope,
                    '>' => Cell::RightSlope,
                    '^' => Cell::UpSlope,
                    'v' => Cell::DownSlope,
                    _ => panic!("Unknown cell type: {}", c),
                })
                .collect()
        })
        .collect()
}

fn print_grid(grid: &Vec<Vec<Cell>>) {
    for row in grid {
        for cell in row {
            match cell {
                Cell::Path => print!("{}", " ".black()),
                Cell::Forest => print!("{}", "#".green()),
                Cell::LeftSlope => print!("{}", "<".blue()),
                Cell::RightSlope => print!("{}", ">".blue()),
                Cell::UpSlope => print!("{}", "^".blue()),
                Cell::DownSlope => print!("{}", "v".blue()),
                Cell::Visited => print!("{}", "*".red()),
            }
        }
        println!();
    }
}
