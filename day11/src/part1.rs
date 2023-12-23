pub fn run(input: &str) -> u32 {
    let grid: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
    let (width, height) = (grid[0].len(), grid.len());
    let mut y_expands = Vec::new();
    let mut x_expands = Vec::new();

    let mut row_empty;
    for y in 0..height {
        row_empty = true;
        for x in 0..width {
            if grid[y][x] == b'#' {
                row_empty = false;
                break;
            }
        }
        if row_empty {
            y_expands.push(y);
        }
    }

    let mut col_empty;
    for x in 0..width {
        col_empty = true;
        for y in 0..height {
            if grid[y][x] == b'#' {
                col_empty = false;
                break;
            }
        }
        if col_empty {
            x_expands.push(x);
        }
    }

    let mut galaxy_coords = Vec::new();
    let mut expand_y = 0;
    let mut expand_x;
    for y in 0..height {
        if y_expands.contains(&y) {
            expand_y += 1;
        }
        expand_x = 0;
        for x in 0..width {
            if x_expands.contains(&x) {
                expand_x += 1;
            }
            if grid[y][x] == b'#' {
                galaxy_coords.push((x + expand_x, y + expand_y));
            }
        }
    }
    let mut total_distance = 0;
    for i in 0..galaxy_coords.len() {
        for j in (i + 1)..galaxy_coords.len() {
            let (x1, y1) = galaxy_coords[i];
            let (x2, y2) = galaxy_coords[j];
            // manhattan distance
            total_distance += (x1 as i32 - x2 as i32).abs() + (y1 as i32 - y2 as i32).abs();
        }
    }
    total_distance as u32
}
