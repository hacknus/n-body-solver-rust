#[derive(PartialEq, PartialOrd)]
pub struct Body {
    pub m: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub vx: f64,
    pub vy: f64,
    pub vz: f64,
    pub ax: f64,
    pub ay: f64,
    pub az: f64,
    pub softening: f64,
}