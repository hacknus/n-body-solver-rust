use crate::body::Body;
use itertools::izip;

use mpi::topology::SystemCommunicator;
use mpi::traits::*;

pub fn calc_direct_force(bodies: &mut Vec<Body>, lower: usize, upper: usize) {
    let g: f64 = 6.67408e-11;
    let softening: f64 = 0.0001;
    let mut x: f64;
    let mut y: f64;
    let mut z: f64;
    let mut r: f64;
    let mut ax: Vec<f64> = vec![0.0; bodies.len()];
    let mut ay: Vec<f64> = vec![0.0; bodies.len()];
    let mut az: Vec<f64> = vec![0.0; bodies.len()];

    for ((i, bi), axi, ayi, azi) in izip!(bodies.iter().enumerate(), &mut ax, &mut ay, &mut az).skip(lower).take(upper - lower) {
        for (j, bj) in bodies.iter().enumerate() {
            if i != j {
                x = bj.x - bi.x;
                y = bj.y - bi.y;
                z = bj.z - bi.z;
                r = (x * x + y * y + z * z + softening * softening).sqrt().powi(3);
                *axi += g * bj.m * x / r;
                *ayi += g * bj.m * y / r;
                *azi += g * bj.m * z / r;
            }
        }
    }
    for (bi, axi, ayi, azi) in izip!(bodies.iter_mut(), &ax, &ay, &az).skip(lower).take(upper - lower) {
        bi.ax = *axi;
        bi.ay = *ayi;
        bi.az = *azi;
    }
}

pub fn leapfrog(bodies: &mut Vec<Body>, dt: f64, lower: usize, upper: usize, world: SystemCommunicator) {
    for bi in bodies.iter_mut().skip(lower).take(upper - lower) {
        bi.x = bi.x + bi.vx * 0.5 * dt;
        bi.y = bi.y + bi.vy * 0.5 * dt;
        bi.z = bi.z + bi.vz * 0.5 * dt;
    }

    for proc in 0..world.size() {
        let ai: usize = (bodies.len() as f32 / world.size() as f32 * proc as f32) as usize;
        let bi: usize = (bodies.len() as f32 / world.size() as f32 * (proc + 1) as f32) as usize;
        world.process_at_rank(proc).broadcast_into(&mut bodies[ai..bi]);
    }
    calc_direct_force(bodies, lower, upper);

    for bi in bodies.iter_mut().skip(lower).take(upper - lower) {
        bi.vx = bi.vx + bi.ax * dt;
        bi.vy = bi.vy + bi.ay * dt;
        bi.vz = bi.vz + bi.az * dt;
        bi.x = bi.x + bi.vx * 0.5 * dt;
        bi.y = bi.y + bi.vy * 0.5 * dt;
        bi.z = bi.z + bi.vz * 0.5 * dt;
    }
    for proc in 0..world.size() {
        let ai: usize = (bodies.len() as f32 / world.size() as f32 * proc as f32) as usize;
        let bi: usize = (bodies.len() as f32 / world.size() as f32 * (proc + 1) as f32) as usize;
        world.process_at_rank(proc).broadcast_into(&mut bodies[ai..bi]);
    }
}

pub fn get_dt(bodies: &Vec<Body>, lower: usize, upper: usize, world: SystemCommunicator) -> f64 {
    let mut dt: Vec<f64> = vec![0.0; upper - lower];
    let softening: f64 = 0.01;
    let min_dt: f64;
    let mut a_mag: f64;
    for (i, bi) in bodies.iter().skip(lower).take(upper - lower).enumerate() {
        a_mag = (bi.ax * bi.ax + bi.ay * bi.ay
            + bi.az * bi.az).sqrt();
        dt[i] = (softening / a_mag).sqrt();
    }
    min_dt = dt.iter().fold(f64::INFINITY, |ai, &bi| ai.min(bi));
    println!("min_dt is {:.32}", min_dt);
    return min_dt;
}