mod body;
mod math_utils;
mod io;

use std::env;
use std::time::Instant;

use crate::body::Body;
use crate::math_utils::{leapfrog, get_dt, calc_direct_force};
use crate::io::{read_csv, write_file};

use mpi::traits::*;
use body::EMPTY_BODY;
type Real = f32;

fn main() {

    let init_start: Instant = Instant::now();

    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let num_processes = world.size();
    let rank = world.rank();

    let mut bodies: Vec<Body> = Vec::new();
    let mut steps: u32 = 0;
    let mut length: usize = 0;

    if rank == 0 {
        println!("Let's calculate some orbits! ");

        let args: Vec<String> = env::args().collect();
        let path = &args[1];
        steps = args[2].parse::<u32>().unwrap();
        bodies = match read_csv(path) {
            Err(e) => panic!("Problem opening the file: {:?}", e),
            Ok(b) => b,
        };
        length = bodies.len();
    }

    world.process_at_rank(0).broadcast_into(&mut steps);
    world.process_at_rank(0).broadcast_into(&mut length);
    if rank != 0 {
        bodies = vec![EMPTY_BODY; length];
    }
    world.process_at_rank(0).broadcast_into(&mut bodies[..]);

    let a: usize = (bodies.len() as f32 / num_processes as f32 * rank as f32) as usize;
    let b: usize = (bodies.len() as f32 / num_processes as f32 * (rank + 1) as f32) as usize;

    println!("total body number: {length}");
    println!("total step number: {steps}");
    println!("rank {rank} from lower {a} to upper {b}.");
    println!("width: {}", b-a);

    let mut dt: Real;
    let mut t: Real = 0.0;

    if rank == 0 {
        println!("init time: {:?}", init_start.elapsed());
    }
    let run_start: Instant = Instant::now();

    // calculate first forces, in order to get initial dt
    calc_direct_force(&mut bodies, a, b);
    for proc in 0..num_processes{
        let ai: usize = (bodies.len() as f32 / num_processes as f32 * proc as f32) as usize;
        let bi: usize = (bodies.len() as f32 / num_processes as f32 * (proc + 1) as f32) as usize;
        world.process_at_rank(proc).broadcast_into(&mut bodies[ai..bi]);
    }
    if rank == 0 {
        println!("run time: {:?}", run_start.elapsed());
    }
    println!("rank {rank} has ax: {}", bodies[4].ax);

    let save_start: Instant = Instant::now();

    if rank == 0 {
        match write_file(&format!("output/out_johannes{:0>5}.dat", 0), &bodies) {
            Err(e) => panic!("Problem writing the output file: {:?}", e),
            Ok(()) => (),
        }
        println!("save time: {:?}", save_start.elapsed());
    }

    return;


    for step in 0..steps {
        // dt = get_dt(&bodies, a, b, world);
        dt = 60.0 * 60.0 * 24.0;
        t += dt;
        leapfrog(&mut bodies, dt, a, b, world);
        println!("calculating step {} at time t+{:.5}", step, t);

        if (rank == 0) && (step % 10 == 0) {
            match write_file(&format!("output/out{:0>5}.dat", step), &bodies) {
                Err(e) => panic!("Problem writing the output file: {:?}", e),
                Ok(()) => (),
            }
        }
        println!("step: {step} on rank {rank}.");
    }

    if rank == 0 {
        println!("runtime: {:?}", run_start.elapsed());
    }
}
