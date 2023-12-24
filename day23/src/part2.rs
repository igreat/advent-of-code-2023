use std::collections::HashMap;

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
    let mut adjacents: HashMap<(isize, isize), Vec<((isize, isize), usize)>> = HashMap::new();
    let start_pos = (1, 0);
    traverse(&mut grid, start_pos, 0, start_pos, &mut adjacents)
}

fn traverse(
    grid: &mut Vec<Vec<Cell>>,
    start: (isize, isize),
    prev_length: u32,
    prev_intersection: (isize, isize),
    adjacents: &mut HashMap<(isize, isize), Vec<((isize, isize), usize)>>,
) -> u32 {
    let mut current;
    let mut temp_current;
    let mut length = prev_length;
    let mut valid_moves = Vec::new();
    let mut grid_copy;
    let mut current_copy;
    let mut length_copy;
    let mut edge_length = 1;

    current = start;
    loop {
        // check for end here as well
        if current.0 == grid[0].len() as isize - 2 && current.1 == grid.len() as isize - 1 {
            // update adjacents
            if let Some(adjacent_nodes) = adjacents.get_mut(&prev_intersection) {
                adjacent_nodes.push((current, edge_length));
            } else {
                adjacents.insert(prev_intersection, vec![((current), edge_length)]);
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
            edge_length += 1;
            grid[current.1 as usize][current.0 as usize] = Cell::Visited;
        } else {
            // update adjacents
            if let Some(adjacent_nodes) = adjacents.get_mut(&prev_intersection) {
                if let Some((_, max_edge_length)) = adjacent_nodes
                    .iter_mut()
                    // see if it contains the current node
                    .find(|(node, _)| node == &current)
                {
                    if edge_length > *max_edge_length {
                        *max_edge_length = edge_length;
                    }
                } else {
                    adjacent_nodes.push((current, edge_length));
                }
            } else {
                adjacents.insert(prev_intersection, vec![((current), edge_length)]);
            }

            // update the other node as well
            if let Some(adjacent_nodes) = adjacents.get_mut(&current) {
                // get the other node as well to update it (undirected graph)
                if let Some((_, max_edge_length)) = adjacent_nodes
                    .iter_mut()
                    // see if it contains the current node
                    .find(|(node, _)| node == &prev_intersection)
                {
                    if edge_length > *max_edge_length {
                        *max_edge_length = edge_length;
                    }
                } else {
                    adjacent_nodes.push((prev_intersection, edge_length));
                }
            } else {
                adjacents.insert(current, vec![(prev_intersection, edge_length)]);
            }

            // check if current is already in adjacents (if so, jump to that node and accumulate length)
            let mut max_length = 0;
            let mut found_intersection = false;
            if let Some(adjacent_nodes) = adjacents.get(&current) {
                // make sure it's not just one node
                if adjacent_nodes.len() != 1 {
                    found_intersection = true;
                    for (node, edge_len) in adjacent_nodes.clone() {
                        // println!("node: {:?}, edge_len: {}", node, edge_len);
                        if matches!(
                            grid[node.1 as usize][node.0 as usize],
                            Cell::Visited | Cell::Forest
                        ) {
                            continue;
                        }
                        length_copy = length + edge_len as u32;
                        grid_copy = grid.clone();
                        // current_copy = node;

                        grid_copy[node.1 as usize][node.0 as usize] = Cell::Visited;
                        // print_grid(&grid_copy);

                        let path_len = traverse(&mut grid_copy, node, length_copy, node, adjacents);
                        if path_len > max_length {
                            max_length = path_len;
                            // println!("max_length: {}", max_length);
                        }
                    }
                }
                // return max_length;
            }

            // update adjacents
            if !found_intersection {
                if let Some(adjacent_nodes) = adjacents.get_mut(&prev_intersection) {
                    if let Some((_, max_edge_length)) = adjacent_nodes
                        .iter_mut()
                        // see if it contains the current node
                        .find(|(node, _)| node == &current)
                    {
                        if edge_length > *max_edge_length {
                            *max_edge_length = edge_length;
                        }
                    } else {
                        adjacent_nodes.push((current, edge_length));
                    }
                } else {
                    adjacents.insert(prev_intersection, vec![((current), edge_length)]);
                }

                // update the other node as well
                if let Some(adjacent_nodes) = adjacents.get_mut(&current) {
                    // get the other node as well to update it (undirected graph)
                    if let Some((_, max_edge_length)) = adjacent_nodes
                        .iter_mut()
                        // see if it contains the current node
                        .find(|(node, _)| node == &prev_intersection)
                    {
                        if edge_length > *max_edge_length {
                            *max_edge_length = edge_length;
                        }
                    } else {
                        adjacent_nodes.push((prev_intersection, edge_length));
                    }
                } else {
                    adjacents.insert(current, vec![(prev_intersection, edge_length)]);
                }
            } else {
                return max_length;
            }

            let mut max_length = 0;
            for direction in valid_moves {
                grid_copy = grid.clone();
                current_copy = current;
                length_copy = length;

                current_copy.0 += direction.0;
                current_copy.1 += direction.1;
                length_copy += 1;
                grid_copy[current_copy.1 as usize][current_copy.0 as usize] = Cell::Visited;

                let path_len = traverse(
                    &mut grid_copy,
                    current_copy,
                    length_copy,
                    current,
                    adjacents,
                );
                if path_len > max_length {
                    max_length = path_len;
                    // println!("max_length: {}", max_length);
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
                Cell::Path => print!("{}", "█".white().on_white()),
                Cell::Forest => print!("{}", "█".black()),
                Cell::Visited => print!("{}", "█".red()),
            }
        }
        println!();
    }
    println!();
}
