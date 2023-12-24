pub fn run(input: &str) -> u32 {
    let particles = parse_input(input);
    println!("particles: {:?}", particles);
    let x_min: i64 = 200000000000000;
    let x_max: i64 = 400000000000000;
    let y_min: i64 = 200000000000000;
    let y_max: i64 = 400000000000000;

    let mut num_intersections = 0;
    for i in 0..particles.len() {
        for j in i + 1..particles.len() {
            let intersection = get_intersection(particles[i], particles[j]);
            if let Some((x, y)) = intersection {
                // check if intersection is within bounds
                if x >= x_min as f64 && x <= x_max as f64 && y >= y_min as f64 && y <= y_max as f64
                {
                    num_intersections += 1;
                }
            }
        }
    }
    println!("num_intersections: {}", num_intersections);
    num_intersections
}

fn get_intersection(
    a: ((f64, f64), (f64, f64)),
    b: ((f64, f64), (f64, f64)),
) -> Option<(f64, f64)> {
    let ((x_a, y_a), (x_vel_a, y_vel_a)) = a;
    let ((x_b, y_b), (x_vel_b, y_vel_b)) = b;

    let a_vel = y_vel_a / x_vel_a;
    let b_vel = y_vel_b / x_vel_b;

    // check if parallel
    if a_vel == b_vel {
        return None;
    }
    let x = (y_b - y_a + a_vel * x_a - b_vel * x_b) / (a_vel - b_vel);
    let y = a_vel * (x - x_a) + y_a;

    if (x < x_a && x_vel_a > 0.0)
        || (x > x_a && x_vel_a < 0.0)
        || (y < y_a && y_vel_a > 0.0)
        || (y > y_a && y_vel_a < 0.0)
        || (x < x_b && x_vel_b > 0.0)
        || (x > x_b && x_vel_b < 0.0)
        || (y < y_b && y_vel_b > 0.0)
        || (y > y_b && y_vel_b < 0.0)
    {
        return None;
    }

    Some((x, y))
}

fn parse_input(input: &str) -> Vec<((f64, f64), (f64, f64))> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split('@').collect();
            let pos: Vec<f64> = parts[0]
                .trim()
                .split(',')
                .map(|p| p.trim().parse::<i64>().unwrap() as f64)
                .collect();
            let vel: Vec<f64> = parts[1]
                .trim()
                .split(',')
                .map(|v| v.trim().parse::<i64>().unwrap() as f64)
                .collect();
            ((pos[0], pos[1]), (vel[0], vel[1]))
        })
        .collect()
}
