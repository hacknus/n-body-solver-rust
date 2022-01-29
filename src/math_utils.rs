use crate::body::Body;
use crate::body::Acc;
use crate::body::EMPTY_ACC;
use crate::Real;

pub fn calc_direct_force(bodies: &mut Vec<Body>) {
    let g: Real = 1.0; //6.67408e-11;
    let softening: Real = 0.0001;
    let mut x: Real;
    let mut y: Real;
    let mut z: Real;
    let mut r: Real;
    let mut temp: Real;

    let mut a: Vec<Acc> = vec![EMPTY_ACC; bodies.len()];
    for (i, (bi, acci)) in bodies.iter().zip(a.iter_mut()).enumerate() {
        for (j, bj) in bodies.iter().enumerate() {
            if i != j {
                x = bj.x - bi.x;
                y = bj.y - bi.y;
                z = bj.z - bi.z;
                r = (x * x + y * y + z * z + softening * softening).sqrt();
                temp = g * bj.m / r.powi(3);
                acci.x += temp * x;
                acci.y += temp * y;
                acci.z += temp * z;
            }
        }
    }
    for (i, (bi, acci)) in bodies.iter_mut().zip(a.iter()).enumerate() {
        bi.ax = acci.x;
        bi.ay = acci.y;
        bi.az = acci.z;
    }
}

pub fn leapfrog(bodies: &mut Vec<Body>, dt: Real) {
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

pub fn get_dt(bodies: &Vec<Body>) -> Real {
    let mut dt: Vec<Real> = vec![0.0; bodies.len()];
    let softening: Real = 0.01;
    let min_dt: Real;
    let mut a_mag: Real;
    for (i, bi) in bodies.iter().enumerate() {
        a_mag = (bi.ax * bi.ax + bi.ay * bi.ay
            + bi.az * bi.az).sqrt();
        dt[i] = (softening / a_mag).sqrt();
    }
    min_dt = dt.iter().fold(Real::INFINITY, |ai, &bi| ai.min(bi));
    println!("min_dt is {:.32}", min_dt);
    return min_dt;
}