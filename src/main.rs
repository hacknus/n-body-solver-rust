mod body;
mod math_utils;
mod io;

use std::env;
use std::time::Instant;

use crate::body::Body;
use crate::math_utils::{leapfrog, get_dt, calc_direct_force};
use crate::io::{read_csv, write_file};

type Real = f64;

fn main() {
    println!("Let's calculate some orbits! ");
    let init_start = Instant::now();

    let (masses, mut x, mut y, mut z, mut vx, mut vy, mut vz) = read_csv("solar_jfc.dat").unwrap();

    let mut ax = vec![0.0; x.len()];
    let mut ay = vec![0.0; x.len()];
    let mut az = vec![0.0; x.len()];
    let mut dt = 60.0 * 60.0 * 24.0;
    let steps = 100;
    println!("init time: {:?}", init_start.elapsed());

    println!("starting calculation...");
    let start = Instant::now();
    // calculate first forces, in order to get initial dt

    for _step in 0..steps {
        leapfrog(&masses, &mut x, &mut y, &mut z, &mut vx, &mut vy, &mut vz, &mut ax, &mut ay, &mut az, dt);
    }
    let elapsed = start.elapsed();
    println!("runtime: {:?}", elapsed);
}
