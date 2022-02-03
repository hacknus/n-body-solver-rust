use crate::Real;
use std::fmt;

#[derive(Debug, Clone, Copy)]
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

impl fmt::Display for Body {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "m = {}, pos = [{},{},{}], vel = [{},{},{}]", self.m, self.x, self.y, self.z, self.vx, self.vy, self.vz)
    }
}