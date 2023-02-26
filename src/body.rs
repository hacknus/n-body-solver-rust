use std::iter::Sum;
use crate::Real;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Body {
    pub id: usize,
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
}

#[derive(Debug, Clone, Copy)]
pub struct Vector {
    pub x: Real,
    pub y: Real,
    pub z: Real,
}

pub const EMPTY_VEC: Vector = Vector { x: 0.0, y: 0.0, z: 0.0 };

impl Vector {
    pub fn norm(&self) -> Real {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

impl Add for &Vector {
    type Output = Vector;
    fn add(self, v: &Vector) -> Vector {
        Vector {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }
}

impl Sub for &Vector {
    type Output = Vector;
    fn sub(self, v: &Vector) -> Vector {
        Vector {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z,
        }
    }
}

impl Mul<Real> for Vector {
    type Output = Vector;
    fn mul(self, a: Real) -> Vector {
        Vector {
            x: self.x * a,
            y: self.y * a,
            z: self.z * a,
        }
    }
}

impl Div<Real> for Vector {
    type Output = Vector;
    fn div(self, a: Real) -> Vector {
        Vector {
            x: self.x / a,
            y: self.y / a,
            z: self.z / a,
        }
    }
}

impl Sum<Self> for Vector {
    fn sum<I>(iter: I) -> Self
        where
            I: Iterator<Item = Self>,
    {
        iter.fold(EMPTY_VEC, |a, b| &a + &b)
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}