use crate::Real;

#[derive(Debug, Clone)]
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