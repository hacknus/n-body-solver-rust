mod body;
mod math_utils;
mod io;

use crate::body::Body;
use crate::math_utils::leapfrog;
use crate::math_utils::get_dt;
use crate::math_utils::calc_direct_force;
use crate::io::read_csv;
use crate::io::write_file;


fn main() {
    println!("Hello, Rust!");
    let mut bodies: Vec<Body> = match read_csv("SolSystData.dat") {
        Err(e) => panic!("Problem opening the file: {:?}", e),
        Ok(b) => b,
    };
    let steps: u32 = 100;
    let mut dt: f64;
    let mut t: f64 = 0.0;

    calc_direct_force(&mut bodies);

    for step in 1..steps {
        dt = get_dt(&mut bodies);
        t += dt;
        leapfrog(&mut bodies, dt);

        println!("body 1: x, y is {:.4}/{:.4}", bodies[0].x, bodies[0].y);

        //snprintf(filename, sizeof(filename), "../output/out_%05d.bin", step);
        //write_file(&mut bodies, filename, dt, t);
        write_file(&format!("output/out{}.dat", step), &bodies);
    }
}
