pub fn run(input: &str) -> i32 {
    let mut numbers = input.lines().map(|line| {
        line.split_whitespace()
            .map(|word| word.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
    });
    let mut total = 0;
    let mut next_nums = Vec::new();
    let mut sign;
    for mut nums in &mut numbers {
        total += nums[0];
        sign = -1;
        loop {
            next_nums.clear();
            for i in 1..nums.len() {
                next_nums.push(nums[i] - nums[i - 1]);
            }
            total += sign * next_nums[0];
            if next_nums.iter().all(|&x| x == 0) || next_nums.len() == 1 {
                break;
            }
            // swap vecs
            std::mem::swap(&mut nums, &mut next_nums);
            sign *= -1;
        }
    }

    total
}
