mod body;
mod math_utils;

use crate::body::Body;
use crate::math_utils::leapfrog;

fn main() {
    println!("Hello, Rust!");
    let mut bodies: Vec<Body> = Vec::new();
    bodies.push(Body { m: 0.0, x: 0.0, y: 0.0, z: 0.0, vx: 1.0, vy: 1.0, vz: 1.0, ax: 0.0, ay: 0.0, az: 0.0, softening: 0.0 });
    for b in &bodies {
        println!("x is {:.32}", b.x);
    }
    leapfrog(&mut bodies, 1.0);
    for b in &bodies {
        println!("x is {:.32}", b.x);
    }
}
