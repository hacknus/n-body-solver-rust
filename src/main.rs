mod body;
mod math_utils;
mod io;

use crate::body::Body;
use crate::math_utils::{leapfrog, get_dt, calc_direct_force};
use crate::io::{read_csv, write_file};

fn main() {
    println!("Hello Rust, let's calculate some orbits!");
    let mut bodies: Vec<Body> = match read_csv("SolSystData.dat") {
        Err(e) => panic!("Problem opening the file: {:?}", e),
        Ok(b) => b,
    };
    let steps: u32 = 10000;
    let mut dt: f64;
    let mut t: f64 = 0.0;

    // calculate first forces, in order to get initial dt
    calc_direct_force(&mut bodies);

    for step in 0..steps {
        dt = get_dt(&bodies);
        dt = 60.0 * 60.0 * 12.0;
        t += dt;
        leapfrog(&mut bodies, dt);
        println!("calculating step {} at time t+{:.5}", step, t);
        match write_file(&format!("output/out{:0>5}.dat", step), &bodies) {
            Err(e) => panic!("Problem writing the output file: {:?}", e),
            Ok(()) => (),
        }
    }
}
