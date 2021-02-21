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
    for i in 0..bodies.len() {
        bodies[i].ax = 0.0;
        bodies[i].ay = 0.0;
        bodies[i].az = 0.0;
        for j in 0..bodies.len() {
            if i != j {
                x = bodies[j].x - bodies[i].x;
                y = bodies[j].y - bodies[i].y;
                z = bodies[j].z - bodies[i].z;
                r = (x * x + y * y + z * z + softening * softening).sqrt();
                bodies[i].ax += g * bodies[j].m * x / r.powi(3);
                bodies[i].ay += g * bodies[j].m * y / r.powi(3);
                bodies[i].az += g * bodies[j].m * z / r.powi(3);
            }
        }
    }
}

pub fn leapfrog(bodies: &mut Vec<Body>, dt: f64) {
    for i in 0..bodies.len() {
        bodies[i].x = bodies[i].x + bodies[i].vx * 0.5 * dt;
        bodies[i].y = bodies[i].y + bodies[i].vy * 0.5 * dt;
        bodies[i].z = bodies[i].z + bodies[i].vz * 0.5 * dt;
    }

    calc_direct_force(bodies);

    for i in 0..bodies.len() {
        bodies[i].vx = bodies[i].vx + bodies[i].ax * dt;
        bodies[i].vy = bodies[i].vy + bodies[i].ay * dt;
        bodies[i].vz = bodies[i].vz + bodies[i].az * dt;
        bodies[i].x = bodies[i].x + bodies[i].vx * 0.5 * dt;
        bodies[i].y = bodies[i].y + bodies[i].vy * 0.5 * dt;
        bodies[i].z = bodies[i].z + bodies[i].vz * 0.5 * dt;
    }
}

pub fn get_dt(bodies: &Vec<Body>) -> f64 {
    let n_p: usize = bodies.len();
    let mut dt: Vec<f64> = vec![0.0; n_p];
    let mut index: usize = 0;
    let softening: f64 = 0.01;
    let min_dt: f64;
    let mut a_mag: f64;
    for b in bodies.iter() {
        a_mag = (b.ax * b.ax + b.ay * b.ay + b.az * b.az).sqrt();
        dt[index] = (softening / a_mag).sqrt();
        index += 1;
    }
    min_dt = dt.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    // println!("min_dt is {:.32}", min_dt);
    return min_dt;
}