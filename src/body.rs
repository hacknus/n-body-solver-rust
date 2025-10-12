// In src/body.rs
use mpi::{
    traits::*,
};
use crate::Real;

#[derive(Equivalence, Clone, Copy)]
pub struct Body {
    pub m: Real,
    pub x: Real,
    pub y: Real,
    pub z: Real,
    pub vx: Real,
    pub vy: Real,
    pub vz: Real,
    pub ax: Real,
    pub ay: Real,
    pub az: Real,
    pub softening: Real,
    pub radius: Real,
    pub agglomerate_id: i32, // -1 for free particles, >=0 for particles in agglomerates
    pub escaped: bool
}

pub const EMPTY_BODY: Body = Body {
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
    radius: 0.0,
    agglomerate_id: -1,
    escaped: false,
};

// Define an Agglomerate struct to manage particle clusters
#[derive(Clone)]
pub struct Agglomerate {
    pub id: i32,
    pub member_indices: Vec<usize>,  // Indices of bodies in the main bodies vector
    pub com: (Real, Real, Real),     // Center of mass
    pub total_mass: Real,
    pub vel: (Real, Real, Real),     // Velocity of center of mass
    pub ang_vel: (Real, Real, Real), // Angular velocity
}

impl Agglomerate {
    pub fn new(id: i32, first_body_idx: usize, bodies: &Vec<Body>) -> Self {
        let body = &bodies[first_body_idx];
        Agglomerate {
            id,
            member_indices: vec![first_body_idx],
            com: (body.x, body.y, body.z),
            total_mass: body.m,
            vel: (body.vx, body.vy, body.vz),
            ang_vel: (0.0, 0.0, 0.0),
        }
    }

    pub fn update_properties(&mut self, bodies: &Vec<Body>) {
        self.total_mass = 0.0;
        let mut com_x = 0.0;
        let mut com_y = 0.0;
        let mut com_z = 0.0;
        let mut vel_x = 0.0;
        let mut vel_y = 0.0;
        let mut vel_z = 0.0;

        for &idx in &self.member_indices {
            let body = &bodies[idx];
            self.total_mass += body.m;
            com_x += body.x * body.m;
            com_y += body.y * body.m;
            com_z += body.z * body.m;
            vel_x += body.vx * body.m;
            vel_y += body.vy * body.m;
            vel_z += body.vz * body.m;
        }

        if self.total_mass > 0.0 {
            com_x /= self.total_mass;
            com_y /= self.total_mass;
            com_z /= self.total_mass;
            vel_x /= self.total_mass;
            vel_y /= self.total_mass;
            vel_z /= self.total_mass;
        }

        self.com = (com_x, com_y, com_z);
        self.vel = (vel_x, vel_y, vel_z);
    }
}

#[derive(Debug, Clone)]
pub struct Acc {
    pub x : Real,
    pub y : Real,
    pub z : Real,
}

pub const EMPTY_ACC: Acc = Acc { x: 0.0, y: 0.0, z: 0.0 };
