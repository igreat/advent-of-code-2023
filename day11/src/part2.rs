pub fn run(input: &str) -> u64 {
    let grid: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
    let (width, height) = (grid[0].len(), grid.len());
    let expansion = 999_999;
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

    let mut galaxy_coords: Vec<(i64, i64)> = Vec::new();
    let mut expand_y: i64 = 0;
    let mut expand_x: i64;
    for y in 0..height {
        if y_expands.contains(&y) {
            expand_y += expansion;
        }
        expand_x = 0;
        for x in 0..width {
            if x_expands.contains(&x) {
                expand_x += expansion;
            }
            if grid[y][x] == b'#' {
                galaxy_coords.push((x as i64 + expand_x, y as i64 + expand_y));
            }
        }
    }
    let mut total_distance: i64 = 0;
    for i in 0..galaxy_coords.len() {
        for j in (i + 1)..galaxy_coords.len() {
            let (x1, y1) = galaxy_coords[i];
            let (x2, y2) = galaxy_coords[j];
            // manhattan distance
            total_distance += (x1 - x2).abs() + (y1 - y2).abs();
        }
    }
    total_distance as u64
}
