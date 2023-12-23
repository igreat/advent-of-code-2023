mod part1;
mod part2;

fn main() {
    let input = std::fs::read_to_string("day23_input.txt").unwrap();
    let result1 = part1::run(&input);
    let result2 = part2::run(&input);

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);
}
