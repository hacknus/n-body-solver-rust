#[path = "body.rs"]
mod body;

use crate::body::Body;

extern crate mpi;

use crate::mpi::collective::CommunicatorCollectives;
use crate::mpi::topology::Communicator;
use crate::mpi::point_to_point as p2p;
use mpi::traits::*;

use std::f64;
use mpi::collective::SystemOperation;
use mpi::topology::{SystemCommunicator, Rank};
use mpi::datatype::{UserDatatype, View, MutView};

pub fn calc_direct_force(bodies: &mut Vec<Body>, world: SystemCommunicator, a: usize, b: usize) {
    let g: f64 = 6.67408e-11;
    let softening: f64 = 0.0001;
    let mut x: f64;
    let mut y: f64;
    let mut z: f64;
    let mut r: f64;

    let t1 = UserDatatype::vector(2,2,3,
                                 &Rank::equivalent_datatype());
    let p : Body = Body {
        m: 0.0,
        x: 0.0,
        y: 0.0,
        z: 0.0,
        vx: 0.0,
        vy: 0.0,
        vz: 0.0,
        ax: 0.0,
        ay: 0.0,
        az: 0.0,
        softening: 0.001,
    };
    let mut send_bodies: Vec<Body> = vec![p; b - a];
    send_bodies.copy_from_slice(&bodies[a..b]);
    let buf0 = unsafe{View::with_count_and_datatype(
        &send_bodies[..], 1, &t1)};
    let mut buf1 = unsafe{MutView::with_count_and_datatype(
        &mut bodies[a..b], 1, &t1)};

    world.all_gather_into(&buf0, &mut buf1);

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
    for i in a..b {
        bodies[i].x = bodies[i].x + bodies[i].vx * 0.5 * dt;
        bodies[i].y = bodies[i].y + bodies[i].vy * 0.5 * dt;
        bodies[i].z = bodies[i].z + bodies[i].vz * 0.5 * dt;
    }

    calc_direct_force(bodies, world, a, b);

    for i in a..b {
        bodies[i].vx = bodies[i].vx + bodies[i].ax * dt;
        bodies[i].vy = bodies[i].vy + bodies[i].ay * dt;
        bodies[i].vz = bodies[i].vz + bodies[i].az * dt;
        bodies[i].x = bodies[i].x + bodies[i].vx * 0.5 * dt;
        bodies[i].y = bodies[i].y + bodies[i].vy * 0.5 * dt;
        bodies[i].z = bodies[i].z + bodies[i].vz * 0.5 * dt;
    }
}

pub fn get_dt(bodies: &Vec<Body>, world: SystemCommunicator, a: usize, b: usize) -> f64 {
    let mut dt: Vec<f64> = vec![0.0; b - a];
    let softening: f64 = 0.01;
    let mut min_dt: f64 = 0.0;
    let mut min_dt_out: f64 = 0.0;
    let mut a_mag: f64;
    for i in a..b {
        a_mag = (bodies[i].ax * bodies[i].ax + bodies[i].ay * bodies[i].ay
            + bodies[i].az * bodies[i].az).sqrt();
        //dt[i-a] = (softening / a_mag).sqrt();
        if i == a {
            min_dt = (softening / a_mag).sqrt();
        } else {
            if min_dt > (softening / a_mag).sqrt() {
                min_dt = (softening / a_mag).sqrt();
            }
        }
    }
    //min_dt = dt.iter().fold(f64::INFINITY, |ai, &bi| ai.min(bi));
    // println!("min_dt is {:.32}", min_dt);
    world.all_reduce_into(&mut min_dt, &mut min_dt_out, SystemOperation::min());
    return min_dt_out;
}