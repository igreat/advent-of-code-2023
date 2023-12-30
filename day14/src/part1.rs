pub fn run(input: &str) -> usize {
    let dish = parse_input(input);
    let mut total = 0;
    let (width, height) = (dish[0].len(), dish.len());
    let mut stops = vec![height; width];
    for (j, &row) in dish.iter().enumerate() {
        for (i, &c) in row.iter().enumerate() {
            match c {
                b'O' => {
                    total += stops[i];
                    stops[i] -= 1;
                }
                b'#' => stops[i] = height - j - 1,
                _ => (),
            }
        }
    }
    total
}

fn parse_input(input: &str) -> Vec<&[u8]> {
    input.lines().map(|l| l.as_bytes()).collect()
}
