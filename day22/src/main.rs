use std::env;
mod part1;
mod part2;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = std::fs::read_to_string("day22_input.txt").unwrap();
    if args.len() > 1 {
        match args[1].as_str() {
            "1" => {
                let result = part1::run(&input);
                println!("Part 1: {}", result);
            }
            "2" => {
                let result = part2::run(&input);
                println!("Part 2: {}", result);
            }
            _ => println!("Invalid argument"),
        }
    } else {
        let result = part1::run(&input);
        println!("Part 1: {}", result);
        let result = part2::run(&input);
        println!("Part 2: {}", result);
    }
}
