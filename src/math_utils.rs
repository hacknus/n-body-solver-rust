use crate::body::Body;
use crate::{Node, Real};
use std::fmt;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;

#[derive(Debug, Clone)]
pub struct Vector {
    pub x: Real,
    pub y: Real,
    pub z: Real,
}

impl Vector {
    pub fn norm(&self) -> Real {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

impl Add for &Vector {
    type Output = Vector;
    fn add(self, v: &Vector) -> Vector {
        Vector { x: self.x + v.x, y: self.y + v.y, z: self.z + v.z }
    }
}

impl Sub for &Vector {
    type Output = Vector;
    fn sub(self, v: &Vector) -> Vector {
        Vector { x: self.x - v.x, y: self.y - v.y, z: self.z - v.z }
    }
}

impl Mul<Real> for Vector {
    type Output = Vector;
    fn mul(self, a: Real) -> Vector {
        Vector { x: self.x * a, y: self.y * a, z: self.z * a }
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{},{},{}]", self.x, self.y, self.z)
    }
}

pub const EMPTY_VEC: Vector = Vector { x: 0.0, y: 0.0, z: 0.0 };

pub fn calc_m_tot(bodies: &Vec<&Body>) -> Real {
    let mut m_tot = 0.0;
    for b in bodies.iter() {
        m_tot += b.m;
    }
    return m_tot;
}

pub fn calc_com(mut bodies: &Vec<&Body>) -> Vector {
    let mut com = EMPTY_VEC;
    let mut m_tot = 0.0;

    for b in bodies.iter() {
        m_tot += b.m;
        com.x += b.x * b.m;
        com.y += b.y * b.m;
        com.z += b.z * b.m;
    }
    com.x /= m_tot;
    com.y /= m_tot;
    com.z /= m_tot;
    return com;
}

pub fn calc_direct_force(bodies: &mut Vec<Body>) {
    let g: Real = 6.67408e-11;
    let softening: Real = 0.0001;
    let mut x: Real;
    let mut y: Real;
    let mut z: Real;
    let mut r: Real;
    let mut temp: Real;

    let mut a: Vec<Vector> = vec![EMPTY_VEC; bodies.len()];
    for (i, (bi, acci)) in bodies.iter().zip(a.iter_mut()).enumerate() {
        for (j, bj) in bodies.iter().enumerate() {
            if i != j {
                x = bj.x - bi.x;
                y = bj.y - bi.y;
                z = bj.z - bi.z;
                r = (x * x + y * y + z * z + softening * softening).sqrt();
                temp = g * bj.m / r.powi(3);
                acci.x += temp * x;
                acci.y += temp * y;
                acci.z += temp * z;
            }
        }
    }
    for (i, (bi, acci)) in bodies.iter_mut().zip(a.iter()).enumerate() {
        bi.ax = acci.x;
        bi.ay = acci.y;
        bi.az = acci.z;
    }
}

pub fn tree_walk(body: &Body, i: usize, node: &Node, theta: Real) -> Vector {
    let mut a = EMPTY_VEC;
    let mut temp: Real = 0.0;
    let softening: Real = 0.0;
    let g: Real = 1.0;
    let x: Real = body.x - node.com.x;
    let y: Real = body.y - node.com.y;
    let z: Real = body.z - node.com.z;
    let mut r: Real = (x * x + y * y + z * z).sqrt();
    if (node.size / r < theta) || (node.is_leaf == true) {
        if node.id == body.id {
            a = EMPTY_VEC;
        } else {
            r = (x * x + y * y + z * z + softening * softening).sqrt();
            temp = g * body.m / r.powi(3);
            a.x = temp * x;
            a.y = temp * y;
            a.z = temp * z;
        }
    } else {
        let len = node.children.len();
        for child in node.children.iter() {
            a = &a + &tree_walk(body, i, child, theta);
        }
    }
    return a;
}

pub fn calc_forces_tree(bodies: &mut Vec<Body>, root: &Node) {
    let theta = 0.0;
    for (i, b) in bodies.iter_mut().enumerate() {
        let a = tree_walk(b, i, root, theta);
        println!("{}", a);
        b.ax = a.x;
        b.ay = a.y;
        b.az = a.z;
    }
}

pub fn leapfrog(bodies: &mut Vec<Body>, dt: Real) {
    for bi in bodies.iter_mut() {
        bi.x = bi.x + bi.vx * 0.5 * dt;
        bi.y = bi.y + bi.vy * 0.5 * dt;
        bi.z = bi.z + bi.vz * 0.5 * dt;
    }

    calc_direct_force(bodies);

    for bi in bodies.iter_mut() {
        bi.vx = bi.vx + bi.ax * dt;
        bi.vy = bi.vy + bi.ay * dt;
        bi.vz = bi.vz + bi.az * dt;
        bi.x = bi.x + bi.vx * 0.5 * dt;
        bi.y = bi.y + bi.vy * 0.5 * dt;
        bi.z = bi.z + bi.vz * 0.5 * dt;
    }
}

pub fn get_dt(bodies: &Vec<Body>) -> Real {
    let mut dt: Vec<Real> = vec![0.0; bodies.len()];
    let softening: Real = 0.01;
    let min_dt: Real;
    let mut a_mag: Real;
    for (i, bi) in bodies.iter().enumerate() {
        a_mag = (bi.ax * bi.ax + bi.ay * bi.ay
            + bi.az * bi.az).sqrt();
        dt[i] = (softening / a_mag).sqrt();
    }
    min_dt = dt.iter().fold(Real::INFINITY, |ai, &bi| ai.min(bi));
    println!("min_dt is {:.32}", min_dt);
    return min_dt;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        let mut a = Vector { x: 1.0, y: 2.0, z: 3.0 };
        let mut b = Vector { x: 4.0, y: 5.0, z: 6.0 };
        let mut c = &a + &b;
        assert_eq!(c, Vector { x: 5.0, y: 7.0, z: 9.0 })
    }
}