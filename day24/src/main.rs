mod part1;

fn main() {
    let input = std::fs::read_to_string("day24_input.txt").unwrap();
    let result1 = part1::run(&input);
    println!("Part 1: {}", result1);
}
