use crate::Real;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Body {
    pub id: u32,
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
    id: 0,
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

impl fmt::Display for Body {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "id = {}, m = {}, pos = [{},{},{}], vel = [{},{},{}]", self.id, self.m, self.x, self.y, self.z, self.vx, self.vy, self.vz)
    }
}