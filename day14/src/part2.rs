use std::collections::HashMap;

pub fn run(input: &str) -> usize {
    let mut dish = parse_input(input);

    let mut dish_cycles = HashMap::new();
    let mut cycle_num = 0;
    loop {
        cycle_num += 1;
        cycle(&mut dish);
        if dish_cycles.contains_key(&dish) {
            let cycle_len = cycle_num - dish_cycles[&dish];
            let cycle_num = (1_000_000_000 - cycle_num - 1) % cycle_len;
            for _ in 0..cycle_num + 1 {
                cycle(&mut dish);
            }
            let load = get_load(&dish);
            return load;
        } else {
            dish_cycles.insert(dish.clone(), cycle_num);
        }
    }
}

fn roll_north(dish: &mut Vec<Vec<u8>>) {
    let (width, height) = (dish[0].len(), dish.len());
    let mut stops = vec![0; width];
    for j in 0..height {
        for i in 0..width {
            match dish[j][i] {
                b'O' => {
                    if j != 0 {
                        dish[j][i] = b'.';
                        dish[stops[i]][i] = b'O';
                    }
                    stops[i] += 1;
                }
                b'#' => stops[i] = j + 1,
                _ => (),
            }
        }
    }
}

fn roll_west(dish: &mut Vec<Vec<u8>>) {
    let (width, height) = (dish[0].len(), dish.len());
    let mut stops = vec![0; height];
    for i in 0..width {
        for j in 0..height {
            match dish[j][i] {
                b'O' => {
                    if i != 0 {
                        dish[j][i] = b'.';
                        dish[j][stops[j]] = b'O';
                    }
                    stops[j] += 1;
                }
                b'#' => stops[j] = i + 1,
                _ => (),
            }
        }
    }
}

fn roll_south(dish: &mut Vec<Vec<u8>>) {
    let (width, height) = (dish[0].len(), dish.len());
    let mut stops = vec![height - 1; width];
    for j in (0..height).rev() {
        for i in 0..width {
            match dish[j][i] {
                b'O' => {
                    if j != height - 1 {
                        dish[j][i] = b'.';
                        dish[stops[i]][i] = b'O';
                    }
                    stops[i] -= 1;
                }
                b'#' => stops[i] = j - 1,
                _ => (),
            }
        }
    }
}

fn roll_east(dish: &mut Vec<Vec<u8>>) {
    let (width, height) = (dish[0].len(), dish.len());
    let mut stops = vec![width - 1; height];
    for i in (0..width).rev() {
        for j in 0..height {
            match dish[j][i] {
                b'O' => {
                    if i != width - 1 {
                        dish[j][i] = b'.';
                        dish[j][stops[j]] = b'O';
                    }
                    stops[j] -= 1;
                }
                b'#' => stops[j] = i - 1,
                _ => (),
            }
        }
    }
}

fn get_load(dish: &Vec<Vec<u8>>) -> usize {
    let (width, height) = (dish[0].len(), dish.len());
    let mut total_load = 0;
    for j in 0..height {
        for i in 0..width {
            if dish[j][i] == b'O' {
                total_load += height - j;
            }
        }
    }
    total_load
}

fn cycle(dish: &mut Vec<Vec<u8>>) {
    roll_north(dish);
    roll_west(dish);
    roll_south(dish);
    roll_east(dish);
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|l| l.as_bytes().to_vec()).collect()
}

fn display_dish(dish: &Vec<Vec<u8>>) {
    for row in dish {
        println!("{}", String::from_utf8(row.clone()).unwrap());
    }
    println!();
}
