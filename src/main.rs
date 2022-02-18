mod body;
mod math_utils;
mod io;
mod node;

use std::env;
use std::time::Instant;

use crate::body::Body;
use crate::math_utils::{leapfrog, get_dt, calc_direct_force, EMPTY_VEC};
use crate::io::{read_csv, write_file};
use crate::node::{Node, EMPTY_NODE, calc_forces_tree, init_root};

type Real = f32;

fn main() {
    println!("Let's calculate some orbits! ");
    let init_start = Instant::now();

    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let steps = &args[2].parse::<u32>().unwrap();
    let mut bodies: Vec<Body>;

    bodies = match read_csv(path) {
        Err(e) => panic!("Problem opening the file: {:?}", e),
        Ok(b) => b,
    };

    let mut bodies_p: Vec<&Body> = Vec::new();
    for b in bodies.iter(){
        bodies_p.push(&b);
    }

    let mut dt: Real;
    let mut t: Real = 0.0;
    println!("init time: {:?}", init_start.elapsed());
    let start_build = Instant::now();

    println!("starting tree building...");
    let mut root: Node = EMPTY_NODE;
    let result = init_root(&mut bodies_p);
    match result {
        Some(tree_root) => root = tree_root,
        None => println!("error building root"),
    }

    root.make_branches(&mut bodies_p);

    println!("building time: {:?}", start_build.elapsed());

    println!("starting calculation...");
    let start = Instant::now();
    // calculate first forces, in order to get initial dt
    calc_forces_tree(&mut bodies, &root);
    println!("runtime: {:?}", start.elapsed());

    match write_file(&format!("output/out{:0>5}_tree.dat", 0), &bodies) {
        Err(e) => panic!("Problem writing the output file: {:?}", e),
        Ok(()) => (),
    }
    return;

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
