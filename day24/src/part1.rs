use rayon::prelude::*;

pub fn run(input: &str) -> u32 {
    let particles = parse_input(input);

    let x_min = 200000000000000f64;
    let x_max = 400000000000000f64;
    let y_min = 200000000000000f64;
    let y_max = 400000000000000f64;

    particles
        .par_iter()
        .enumerate()
        .map(|(i, &particle_i)| {
            particles[i + 1..]
                .iter()
                .filter(|&&particle_j| {
                    is_intersection(particle_i, particle_j, x_min, x_max, y_min, y_max)
                })
                .count()
        })
        .sum::<usize>() as u32
}

fn is_intersection(
    a: ((f64, f64), f64, f64),
    b: ((f64, f64), f64, f64),
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
) -> bool {
    let ((x_a, y_a), y_vel_a, a_vel) = a;
    let ((x_b, y_b), y_vel_b, b_vel) = b;

    // check if parallel
    if a_vel == b_vel {
        return false;
    }

    let mut x = y_b - y_a + a_vel * x_a - b_vel * x_b;
    if !(x.abs() >= x_min * (a_vel - b_vel).abs() && x.abs() <= x_max * (a_vel - b_vel).abs()) {
        return false;
    }
    x /= a_vel - b_vel;
    let y = a_vel * (x - x_a) + y_a;

    if !(x >= x_min && x <= x_max && y >= y_min && y <= y_max)
        || (y < y_a && y_vel_a > 0.0)
        || (y > y_a && y_vel_a < 0.0)
        || (y < y_b && y_vel_b > 0.0)
        || (y > y_b && y_vel_b < 0.0)
    {
        return false;
    }

    true
}

fn parse_input(input: &str) -> Vec<((f64, f64), f64, f64)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(" @ ");
            let pos: Vec<f64> = parts
                .next()
                .unwrap()
                .split(", ")
                .map(|p| p.parse::<i64>().unwrap() as f64)
                .collect();
            let vel: Vec<f64> = parts
                .next()
                .unwrap()
                .split(", ")
                .map(|v| v.parse::<i64>().unwrap() as f64)
                .collect();
            let a_vel = vel[1] / vel[0];

            ((pos[0], pos[1]), vel[1], a_vel)
        })
        .collect()
}
