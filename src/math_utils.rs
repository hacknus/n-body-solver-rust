#[path = "body.rs"]
mod body;

use crate::body::Body;

extern crate mpi;

use crate::mpi::collective::CommunicatorCollectives;
use crate::mpi::topology::Communicator;
use crate::mpi::topology::Rank;
use crate::mpi::point_to_point::Destination;
use mpi::traits::*;

use std::f64;
use mpi::collective::SystemOperation;
use mpi::topology::SystemCommunicator;

pub fn calc_direct_force(bodies: &mut Vec<Body>, world: SystemCommunicator, a: usize, b: usize) {
    let g: f64 = 6.67408e-11;
    let softening: f64 = 0.0001;
    let mut x: f64;
    let mut y: f64;
    let mut z: f64;
    let mut r: f64;
    let mut ai: usize;
    let mut bi: usize;

    if world.rank() != 0 {
        for i in a..b {
            // this is ugly...
            world.process_at_rank(0).send(&bodies[i].x);
            world.process_at_rank(0).send(&bodies[i].y);
            world.process_at_rank(0).send(&bodies[i].z);
            world.process_at_rank(0).send(&bodies[i].vx);
            world.process_at_rank(0).send(&bodies[i].vy);
            world.process_at_rank(0).send(&bodies[i].vz);
            world.process_at_rank(0).send(&bodies[i].ax);
            world.process_at_rank(0).send(&bodies[i].ay);
            world.process_at_rank(0).send(&bodies[i].az);
        }
    } else {
        for i in 1..world.size() as usize{
            ai = bodies.len() / world.size() as usize * i;
            bi = bodies.len() / world.size() as usize * (i+1);
            let mut buf : f64 = 0.0;
            for j in ai..bi {
                // this is ugly...
                world.process_at_rank(i as i32).receive_into(&mut buf);
                bodies[j].x = buf as f64;
                world.process_at_rank(i as i32).receive_into(&mut buf);
                bodies[j].y = buf as f64;
                world.process_at_rank(i as i32).receive_into(&mut buf);
                bodies[j].z = buf as f64;
                world.process_at_rank(i as i32).receive_into(&mut buf);
                bodies[j].vx = buf as f64;
                world.process_at_rank(i as i32).receive_into(&mut buf);
                bodies[j].vy = buf as f64;
                world.process_at_rank(i as i32).receive_into(&mut buf);
                bodies[j].vz = buf as f64;
                world.process_at_rank(i as i32).receive_into(&mut buf);
                bodies[j].ax = buf as f64;
                world.process_at_rank(i as i32).receive_into(&mut buf);
                bodies[j].ay = buf as f64;
                world.process_at_rank(i as i32).receive_into(&mut buf);
                bodies[j].az = buf as f64;
            }
        }
        // broadcast bodies to all processes from root
        for i in 0..bodies.len(){
            world.process_at_rank(0).broadcast_into(&mut bodies[i].m);
            world.process_at_rank(0).broadcast_into(&mut bodies[i].x);
            world.process_at_rank(0).broadcast_into(&mut bodies[i].y);
            world.process_at_rank(0).broadcast_into(&mut bodies[i].z);
            world.process_at_rank(0).broadcast_into(&mut bodies[i].vx);
            world.process_at_rank(0).broadcast_into(&mut bodies[i].vy);
            world.process_at_rank(0).broadcast_into(&mut bodies[i].vz);
        }
    }


    for i in a..b {
        bodies[i].ax = 0.0;
        bodies[i].ay = 0.0;
        bodies[i].az = 0.0;
        for j in 0..bodies.len() {
            if i != j {
                x = bodies[j].x - bodies[i].x;
                y = bodies[j].y - bodies[i].y;
                z = bodies[j].z - bodies[i].z;
                r = (x * x + y * y + z * z + softening * softening).sqrt();
                bodies[i].ax += g * bodies[j].m * x / r.powi(3);
                bodies[i].ay += g * bodies[j].m * y / r.powi(3);
                bodies[i].az += g * bodies[j].m * z / r.powi(3);
            }
        }
    }
}

pub fn leapfrog(bodies: &mut Vec<Body>, dt: f64, world: SystemCommunicator, a: usize, b: usize) {
    for i in 0..bodies.len() {
        bodies[i].x = bodies[i].x + bodies[i].vx * 0.5 * dt;
        bodies[i].y = bodies[i].y + bodies[i].vy * 0.5 * dt;
        bodies[i].z = bodies[i].z + bodies[i].vz * 0.5 * dt;
    }

    calc_direct_force(bodies, world, a, b);

    for i in 0..bodies.len() {
        bodies[i].vx = bodies[i].vx + bodies[i].ax * dt;
        bodies[i].vy = bodies[i].vy + bodies[i].ay * dt;
        bodies[i].vz = bodies[i].vz + bodies[i].az * dt;
        bodies[i].x = bodies[i].x + bodies[i].vx * 0.5 * dt;
        bodies[i].y = bodies[i].y + bodies[i].vy * 0.5 * dt;
        bodies[i].z = bodies[i].z + bodies[i].vz * 0.5 * dt;
    }
}

pub fn get_dt(bodies: &Vec<Body>, world: SystemCommunicator) -> f64 {
    let rank = world.rank();
    let n_p: usize = bodies.len();
    let mut dt: Vec<f64> = vec![0.0; n_p];
    let mut index: usize = 0;
    let softening: f64 = 0.01;
    let mut min_dt: f64;
    let mut a_mag: f64;
    for b in bodies.iter() {
        a_mag = (b.ax * b.ax + b.ay * b.ay + b.az * b.az).sqrt();
        dt[index] = (softening / a_mag).sqrt();
        index += 1;
    }
    min_dt = dt.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    // println!("min_dt is {:.32}", min_dt);
    world.all_reduce_into(&rank, &mut min_dt, SystemOperation::min());
    return min_dt;
}