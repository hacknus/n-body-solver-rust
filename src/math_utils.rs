#[path = "body.rs"]
mod body;

use crate::body::Body;

use std::f64;

pub fn calc_direct_force(bodies: &mut Vec<Body>) {
    let g: f64 = 6.67408e-11;
    let softening: f64 = 0.0001;
    let mut x: f64;
    let mut y: f64;
    let mut z: f64;
    let mut r: f64;
    let mut ax: Vec<f64> = vec![0.0; bodies.len()];
    let mut ay: Vec<f64> = vec![0.0; bodies.len()];
    let mut az: Vec<f64> = vec![0.0; bodies.len()];

    for (i, bi) in bodies.iter().enumerate() {
        for (j, bj) in bodies.iter().enumerate() {
            if i != j {
                x = bj.x - bi.x;
                y = bj.y - bi.y;
                z = bj.z - bi.z;
                r = (x * x + y * y + z * z + softening * softening).sqrt();
                ax[i] += g * bj.m * x / r.powi(3);
                ay[i] += g * bj.m * y / r.powi(3);
                az[i] += g * bj.m * z / r.powi(3);
            }
        }
    }
    for (i, bi) in bodies.iter_mut().enumerate() {
        bi.ax = ax[i];
        bi.ay = ay[i];
        bi.az = az[i];
    }
}

pub fn leapfrog(bodies: &mut Vec<Body>, dt: f64) {
    for bi in bodies.iter_mut() {
        bi.x = bi.x + bi.vx * 0.5 * dt;
        bi.y = bi.y + bi.vy * 0.5 * dt;
        bi.z = bi.z + bi.vz * 0.5 * dt;
    }

    calc_direct_force(bodies);

    for bi in bodies.iter_mut() {
        bi.vx = bi.vx + bi.ax * dt;
        bi.vy = bi.vy + bi.ay * dt;
        bi.vz = bi.vz + bi.az * dt;
        bi.x = bi.x + bi.vx * 0.5 * dt;
        bi.y = bi.y + bi.vy * 0.5 * dt;
        bi.z = bi.z + bi.vz * 0.5 * dt;
    }
}

pub fn get_dt(bodies: &Vec<Body>) -> f64 {
    let mut dt: Vec<f64> = vec![0.0; bodies.len()];
    let softening: f64 = 0.01;
    let min_dt: f64;
    let mut a_mag: f64;
    for (i, bi) in bodies.iter().enumerate() {
        a_mag = (bi.ax * bi.ax + bi.ay * bi.ay
            + bi.az * bi.az).sqrt();
        dt[i] = (softening / a_mag).sqrt();
    }
    min_dt = dt.iter().fold(f64::INFINITY, |ai, &bi| ai.min(bi));
    println!("min_dt is {:.32}", min_dt);
    return min_dt;
}