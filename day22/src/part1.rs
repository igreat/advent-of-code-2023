pub fn run(input: &str) -> usize {
    let mut blocks = parse_input(input);
    let mut occupied = drop_blocks(&mut blocks);

    let mut z_levels = vec![vec![]; occupied.len()];
    for block in blocks {
        z_levels[block.0.z].push(block);
    }

    let mut removable_blocks = z_levels.last().unwrap().len();
    for z in (0..z_levels.len() - 1).rev() {
        for block in &z_levels[z] {
            // remove block from occupied
            for y in block.0.y..=block.1.y {
                for x in block.0.x..=block.1.x {
                    occupied[block.1.z][y][x] = false;
                }
            }

            // check if anything right above is going to fall
            if block.1.z + 1 >= occupied.len() {
                removable_blocks += 1;
                continue;
            }
            if z_levels[block.1.z + 1].iter().all(|above| {
                (above.0.y..=above.1.y)
                    .any(|y| (above.0.x..=above.1.x).any(|x| occupied[block.1.z][y][x]))
            }) {
                removable_blocks += 1;
            }

            // add block back to occupied
            for y in block.0.y..=block.1.y {
                for x in block.0.x..=block.1.x {
                    occupied[block.1.z][y][x] = true;
                }
            }
        }
    }
    removable_blocks
}

fn drop_blocks(blocks: &mut Vec<(Point, Point)>) -> Vec<Vec<Vec<bool>>> {
    let max_x = blocks.iter().map(|b| b.1.x).max().unwrap();
    let max_y = blocks.iter().map(|b| b.1.y).max().unwrap();
    let mut occupied = vec![vec![vec![false; max_x + 1]; max_y + 1]];
    let mut current_bottom = vec![vec![0; max_x + 1]; max_y + 1];

    // sort blocks by z
    blocks.sort_by(|a, b| a.0.z.cmp(&b.0.z));
    for block in blocks {
        let mut max = usize::MIN;
        for y in block.0.y..=block.1.y {
            for x in block.0.x..=block.1.x {
                if current_bottom[y][x] > max {
                    max = current_bottom[y][x];
                }
            }
        }
        // move block to current_bottom
        let height = block.1.z - block.0.z;
        block.0.z = max + 1;
        block.1.z = block.0.z + height;
        for y in block.0.y..=block.1.y {
            for x in block.0.x..=block.1.x {
                current_bottom[y][x] = block.1.z;

                // mark as occupied
                while block.1.z >= occupied.len() {
                    occupied.push(vec![vec![false; max_x + 1]; max_y + 1]);
                    occupied.last_mut().unwrap()[y][x] = true;
                }
                occupied[block.1.z][y][x] = true;
            }
        }
    }
    occupied
}

#[derive(Debug, Clone)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

fn parse_input(input: &str) -> Vec<(Point, Point)> {
    input
        .lines()
        .map(|line| {
            let (start, end) = line.split_once("~").unwrap();
            let start: Vec<usize> = start
                .split(",")
                .filter_map(|s| s.parse::<usize>().ok())
                .collect();
            let end: Vec<usize> = end
                .split(",")
                .filter_map(|s| s.parse::<usize>().ok())
                .collect();

            (
                Point {
                    x: start[0],
                    y: start[1],
                    z: start[2],
                },
                Point {
                    x: end[0],
                    y: end[1],
                    z: end[2],
                },
            )
        })
        .collect()
}
