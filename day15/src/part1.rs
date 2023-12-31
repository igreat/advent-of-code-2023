pub fn run(input: &str) -> usize {
    let mut total = 0;
    for string in input.split(',') {
        total += hash(string.as_bytes());
    }
    total
}

fn hash(word: &[u8]) -> usize {
    let mut result: usize = 0;
    for &b in word {
        result = ((result + b as usize) * 17) % 256;
    }
    result
}
