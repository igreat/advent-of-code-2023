pub fn run(input: &str) -> usize {
    let mut map = HashMap::new();
    for string in input.split(',') {
        if let Some((key, value)) = string.split_once('=') {
            map.insert(key.to_string(), value.parse().unwrap());
        } else {
            map.remove(&string[..string.len() - 1]);
        }
    }

    let mut total = 0;
    for (i, bucket) in map.buckets.iter().enumerate() {
        if bucket.is_empty() {
            continue;
        }
        for (j, (_, value)) in bucket.iter().enumerate() {
            total += (i + 1) * (j + 1) * value;
        }
    }
    total
}

#[derive(Debug)]
struct HashMap {
    buckets: [Vec<(String, usize)>; 256],
}

impl HashMap {
    fn new() -> Self {
        const EMPTY_BUCKET: Vec<(String, usize)> = Vec::new();
        HashMap {
            buckets: [EMPTY_BUCKET; 256],
        }
    }

    fn insert(&mut self, key: String, value: usize) {
        let index = hash(key.as_bytes());
        // self.buckets[index].push((key, value));
        for i in 0..self.buckets[index].len() {
            if self.buckets[index][i].0 == key {
                self.buckets[index][i].1 = value;
                return;
            }
        }
        self.buckets[index].push((key, value));
    }

    fn remove(&mut self, key: &str) -> Option<usize> {
        let index = hash(key.as_bytes());
        for i in 0..self.buckets[index].len() {
            if self.buckets[index][i].0 == key {
                return Some(self.buckets[index].remove(i).1);
            }
        }
        None
    }
}

fn hash(word: &[u8]) -> usize {
    let mut result: usize = 0;
    for &b in word {
        result = ((result + b as usize) * 17) % 256;
    }
    result
}
