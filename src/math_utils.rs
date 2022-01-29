use crate::body::Body;
use crate::body::Acc;
use body::EMPTY_ACC;

use mpi::topology::SystemCommunicator;
use mpi::traits::*;
use crate::body;
use crate::Real;

pub fn calc_direct_force(bodies: &mut Vec<Body>, lower: usize, upper: usize) {
    let g: Real = 6.67408e-11;
    let softening: Real = 0.0001;
    let mut x: Real;
    let mut y: Real;
    let mut z: Real;
    let mut r: Real;

    let mut a: Vec<Acc> = vec![EMPTY_ACC; bodies.len()];
    for ((i, bi), acci) in bodies.iter().enumerate().zip(a.iter_mut()).skip(lower).take(upper - lower) {
        for (j, bj) in bodies.iter().enumerate() {
            if i != j {
                x = bj.x - bi.x;
                y = bj.y - bi.y;
                z = bj.z - bi.z;
                r = (x * x + y * y + z * z + softening * softening).sqrt().powi(3);
                acci.x += g * bj.m * x / r;
                acci.y += g * bj.m * y / r;
                acci.z += g * bj.m * z / r;
            }
        }
    }
    for (bi, acci) in bodies.iter_mut().zip( a.iter()).skip(lower).take(upper - lower) {
        bi.ax = acci.x;
        bi.ay = acci.y;
        bi.az = acci.z;
    }
}

pub fn leapfrog(bodies: &mut Vec<Body>, dt: Real, lower: usize, upper: usize, world: SystemCommunicator) {
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

pub fn get_dt(bodies: &Vec<Body>, lower: usize, upper: usize, world: SystemCommunicator) -> Real {
    let mut dt: Vec<Real> = vec![0.0; upper - lower];
    let softening: Real = 0.01;
    let min_dt: Real;
    let mut a_mag: Real;
    for (i, bi) in bodies.iter().skip(lower).take(upper - lower).enumerate() {
        a_mag = (bi.ax * bi.ax + bi.ay * bi.ay
            + bi.az * bi.az).sqrt();
        dt[i] = (softening / a_mag).sqrt();
    }
    min_dt = dt.iter().fold(Real::INFINITY, |ai, &bi| ai.min(bi));
    println!("min_dt is {:.32}", min_dt);
    return min_dt;
}