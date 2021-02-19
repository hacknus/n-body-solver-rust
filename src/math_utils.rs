#[path = "body.rs"]
mod body;

use crate::body::Body;

pub fn leapfrog(bodies: &mut Vec<Body>, dt: f64) {
    for b in bodies.iter_mut() {
        b.x = b.x + b.vx * 0.5 * dt;
        b.y = b.y + b.vy * 0.5 * dt;
        b.z = b.z + b.vz * 0.5 * dt;
    }

    // calc_direct_force(bodies);

    for b in bodies.iter_mut() {
        b.vx = b.vx + b.ax * dt;
        b.vy = b.vy + b.ay * dt;
        b.vz = b.vz + b.az * dt;
        b.x = b.x + b.vx * 0.5 * dt;
        b.y = b.y + b.vy * 0.5 * dt;
        b.z = b.z + b.vz * 0.5 * dt;
    }
}

