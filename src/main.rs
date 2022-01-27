mod body;
mod math_utils;
mod io;

use std::env;
use std::time::Instant;

use crate::body::Body;
use crate::math_utils::{leapfrog, get_dt, calc_direct_force};
use crate::io::{read_csv, write_file};

fn main() {
    let start = Instant::now();
    println!("Let's calculate some orbits! ");

    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let steps = &args[2].parse::<u32>().unwrap();
    let mut bodies: Vec<Body>;

    bodies = match read_csv(path) {
        Err(e) => panic!("Problem opening the file: {:?}", e),
        Ok(b) => b,
    };


    let mut dt: f64;
    let mut t: f64 = 0.0;

    // calculate first forces, in order to get initial dt
    calc_direct_force(&mut bodies);

    for step in 0..*steps {
        dt = get_dt(&bodies);
        dt = 60.0 * 60.0 * 24.0;
        t += dt;
        leapfrog(&mut bodies, dt);
        println!("calculating step {} at time t+{:.5}", step, t);
        if step % 10 == 0 {
            match write_file(&format!("output/out{:0>5}.dat", step), &bodies) {
                Err(e) => panic!("Problem writing the output file: {:?}", e),
                Ok(()) => (),
            }
        }
    }
    println!("runtime: {:?}", start.elapsed());
}
