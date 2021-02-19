#[path = "body.rs"]
mod body;

use crate::body::Body;
use std::f64;


pub fn calc_direct_force(bodies: &mut Vec<Body>) {
    let g: f64 = 1.0;
    let softening: f64 = 0.01;
    let mut x: f64;
    let mut y: f64;
    let mut z: f64;
    let mut r: f64;
    //for b in bodies.iter_mut() {
    for i in 0..bodies.len() {
        bodies[i].ax = 0.0;
        bodies[i].ay = 0.0;
        bodies[i].az = 0.0;
        //for p in bodies.iter() {
        for j in 0..bodies.len() {
            if i != j {
                x = bodies[i].x - bodies[j].x;
                y = bodies[i].y - bodies[j].y;
                z = bodies[i].z - bodies[j].z;
                r = (x.powi(2) + y.powi(2) + z.powi(2)
                    + softening.powi(2)).powf(1.5);
                bodies[i].ax -=
                    g * bodies[j].m * x / r;
                bodies[i].ay -=
                    g * bodies[j].m * y / r;
                bodies[i].az -=
                    g * bodies[j].m * z / r;
            }
        }
    }
}

pub fn leapfrog(bodies: &mut Vec<Body>, dt: f64) {
    for b in bodies.iter_mut() {
        b.x = b.x + b.vx * 0.5 * dt;
        b.y = b.y + b.vy * 0.5 * dt;
        b.z = b.z + b.vz * 0.5 * dt;
    }

    calc_direct_force(bodies);

    for b in bodies.iter_mut() {
        b.vx = b.vx + b.ax * dt;
        b.vy = b.vy + b.ay * dt;
        b.vz = b.vz + b.az * dt;
        b.x = b.x + b.vx * 0.5 * dt;
        b.y = b.y + b.vy * 0.5 * dt;
        b.z = b.z + b.vz * 0.5 * dt;
    }

}

pub fn get_dt(bodies: &mut Vec<Body>) -> f64 {
    let n_p: usize = bodies.len();
    let mut dt: Vec<f64> = vec![0.0; n_p];
    let mut index: usize = 0;
    let softening: f64 = 0.01;
    let min_dt: f64;
    let mut a_mag: f64;
    for b in bodies.iter() {
        a_mag = (b.ax.powi(2) + b.ay.powi(2) + b.az.powi(2)).sqrt();
        dt[index] = 0.1 * (softening / a_mag).sqrt();
        index += 1;
    }
    min_dt = dt.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    println!("min_dt is {:.32}", min_dt);
    return min_dt;
}