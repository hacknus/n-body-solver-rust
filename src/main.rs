mod body;
mod math_utils;
mod io;

extern crate mpi;

use mpi::request::WaitGuard;
use mpi::traits::*;

use std::env;

use crate::body::Body;
use crate::math_utils::{leapfrog, get_dt, calc_direct_force};
use crate::io::{read_csv, write_file};

fn main() {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let size = world.size();
    let rank = world.rank();

    println!("Let's calculate some orbits! I am on rank {} ", rank);

    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let steps = &args[2].parse::<u32>().unwrap();
    let mut bodies: Vec<Body>;

    // this is ugly...
    // only let root process read in the file!

    bodies = match read_csv(path) {
        Err(e) => panic!("Problem opening the file: {:?}", e),
        Ok(b) => b,
    };



    let mut dt: f64;
    let mut t: f64 = 0.0;

    let size = world.size();
    let rank = world.rank();
    let a: usize = bodies.len() / size as usize * rank as usize;
    let b: usize = bodies.len() / size  as usize * (rank + 1) as usize;

    // calculate first forces, in order to get initial dt
    calc_direct_force(&mut bodies, world, a, b);

    for step in 0..*steps {
        dt = get_dt(&bodies, world);
        dt = 60.0 * 60.0 * 24.0;
        t += dt;
        leapfrog(&mut bodies, dt, world, a, b);
        if rank == 0 {
            println!("calculating step {} at time t+{:.5}", step, t);
            match write_file(&format!("output/out{:0>5}.dat", step), &bodies) {
                Err(e) => panic!("Problem writing the output file: {:?}", e),
                Ok(()) => (),
            }
        }
    }
}
