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
};

#[derive(Debug, Clone)]
pub struct Acc {
    pub x : Real,
    pub y : Real,
    pub z : Real,
}

pub const EMPTY_ACC: Acc = Acc { x: 0.0, y: 0.0, z: 0.0 };