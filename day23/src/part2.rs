use colored::Colorize;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Cell {
    Path,
    Forest,
    Visited,
}

pub fn run(input: &str) -> u32 {
    // println!("input: {}", input);
    let mut grid = parse_input(input);
    grid[0][1] = Cell::Visited;
    traverse(&mut grid, (1, 0), 0)
}

static mut MAX_LENGTH: u32 = 0;
fn traverse(grid: &mut Vec<Vec<Cell>>, start: (isize, isize), prev_length: u32) -> u32 {
    if start.0 == grid[0].len() as isize - 2 && start.1 == grid.len() as isize - 1 {
        if prev_length > unsafe { MAX_LENGTH } {
            unsafe {
                MAX_LENGTH = prev_length;
                println!("max length: {}", MAX_LENGTH);
            }
        }
        return prev_length;
    }

    let mut current;
    let mut temp_current;
    let mut length = prev_length;
    let mut valid_moves = Vec::new();
    current = start;
    loop {
        // check for end here as well
        if current.0 == grid[0].len() as isize - 2 && current.1 == grid.len() as isize - 1 {
            if length > unsafe { MAX_LENGTH } {
                unsafe {
                    MAX_LENGTH = length;
                    println!("max length: {}", MAX_LENGTH);
                }
            }
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
                Cell::Forest | Cell::Visited => continue,
            }
        }

        if valid_moves.len() == 0 {
            return 0;
        }

        if valid_moves.len() == 1 {
            let direction = valid_moves[0];
            current.0 += direction.0;
            current.1 += direction.1;
            length += 1;
            grid[current.1 as usize][current.0 as usize] = Cell::Visited;
        } else {
            let mut max_length = length;
            for direction in valid_moves {
                let mut grid_copy = grid.clone();
                let mut current_copy = current;
                let mut length_copy = length;

                current_copy.0 += direction.0;
                current_copy.1 += direction.1;
                length_copy += 1;
                grid_copy[current_copy.1 as usize][current_copy.0 as usize] = Cell::Visited;

                let result = traverse(&mut grid_copy, current_copy, length_copy);
                if result > max_length {
                    max_length = result;
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
                    '#' => Cell::Forest,
                    _ => Cell::Path,
                })
                .collect()
        })
        .collect()
}

fn print_grid(grid: &Vec<Vec<Cell>>) {
    for row in grid {
        for cell in row {
            match cell {
                Cell::Path => print!(" "),
                Cell::Forest => print!("{}", "#".black()),
                Cell::Visited => print!("{}", "*".red()),
            }
        }
        println!();
    }
}
