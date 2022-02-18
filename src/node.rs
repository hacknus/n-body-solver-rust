use crate::Real;
use crate::body::{Body, EMPTY_BODY};
use crate::math_utils::{EMPTY_VEC, Vector};
use crate::math_utils::calc_com;
use crate::math_utils::calc_m_tot;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Node {
    pub id: u32,
    pub children: Vec<Node>,
    pub m: Real,
    pub com: Vector,
    pub min: Vector,
    pub max: Vector,
    pub size: Real,
    pub is_leaf: bool,
}

pub const EMPTY_NODE: Node = Node {
    id: 0,
    children: vec![],
    m: 0.0,
    com: EMPTY_VEC,
    min: EMPTY_VEC,
    max: EMPTY_VEC,
    size: 0.0,
    is_leaf: false,
};

impl Node {
    pub fn new_node(mut bodies: &mut Vec<&Body>, min: Vector, max: Vector) -> Option<Node> {
        let mut is_leaf = false;
        let mut id = 0;

        if bodies.len() > 0 {
            let size = (&max - &min).norm();
            let m_tot = calc_m_tot(&bodies);
            let com = calc_com(&bodies);
            if bodies.len() == 1 {
                id = bodies[0].id;
                is_leaf = true;
            }
            let new_node = Node { id, children: Vec::new(), m: m_tot, com, min, max, size, is_leaf };
            Some(new_node)
        } else {
            None
        }
    }

    pub fn make_branches(&mut self, bodies: &mut Vec<&Body>) {
        if self.is_leaf == true {
            return;
        }
        let width = (&self.max.x - &self.min.x) * 0.5;
        let mut min: Vector;
        let mut max: Vector;
        let offset = Vector { x: width, y: width, z: width };
        let mut bodies_lower_east: Vec<&Body> = Vec::new();
        let mut bodies_lower_south: Vec<&Body> = Vec::new();
        let mut bodies_lower_west: Vec<&Body> = Vec::new();
        let mut bodies_lower_north: Vec<&Body> = Vec::new();
        let mut bodies_upper_east: Vec<&Body> = Vec::new();
        let mut bodies_upper_south: Vec<&Body> = Vec::new();
        let mut bodies_upper_west: Vec<&Body> = Vec::new();
        let mut bodies_upper_north: Vec<&Body> = Vec::new();
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    min = &self.min + &Vector { x: i as f32 * width, y: j as f32 * width, z: k as f32 * width };
                    max = &self.min + &offset;
                    max = &max + &Vector { x: i as f32 * width, y: j as f32 * width, z: k as f32 * width };
                    match (i, j, k) {
                        (0, 0, 0) => {
                            for b in bodies.into_iter() {
                                if min.x <= b.x && b.x < max.x && min.y <= b.y && b.y < max.y && min.z <= b.z && b.z < max.z {
                                    bodies_lower_east.push(b);
                                }
                            }
                            let result = Node::new_node(&mut bodies_lower_east, min, max);
                            match result {
                                Some(node) => self.children.push(node),
                                None => (),
                            }
                        }
                        (0, 1, 0) => {
                            for b in bodies.into_iter() {
                                if min.x <= b.x && b.x < max.x && min.y <= b.y && b.y < max.y && min.z <= b.z && b.z < max.z {
                                    bodies_lower_south.push(b);
                                }
                            }
                            let result = Node::new_node(&mut bodies_lower_south, min, max);
                            match result {
                                Some(node) => self.children.push(node),
                                None => (),
                            }
                        }
                        (1, 0, 0) => {
                            for b in bodies.into_iter() {
                                if min.x <= b.x && b.x < max.x && min.y <= b.y && b.y < max.y && min.z <= b.z && b.z < max.z {
                                    bodies_lower_west.push(b);
                                }
                            }
                            let result = Node::new_node(&mut bodies_lower_west, min, max);
                            match result {
                                Some(node) => self.children.push(node),
                                None => (),
                            }
                        }
                        (1, 1, 0) => {
                            for b in bodies.into_iter() {
                                if min.x <= b.x && b.x < max.x && min.y <= b.y && b.y < max.y && min.z <= b.z && b.z < max.z {
                                    bodies_lower_north.push(b);
                                }
                            }
                            let result = Node::new_node(&mut bodies_lower_north, min, max);
                            match result {
                                Some(node) => self.children.push(node),
                                None => (),
                            }
                        }
                        (1, 1, 1) => {
                            for b in bodies.into_iter() {
                                if min.x <= b.x && b.x < max.x && min.y <= b.y && b.y < max.y && min.z <= b.z && b.z < max.z {
                                    bodies_upper_east.push(b);
                                }
                            }
                            let result = Node::new_node(&mut bodies_upper_east, min, max);
                            match result {
                                Some(node) => self.children.push(node),
                                None => (),
                            }
                        }
                        (0, 1, 1) => {
                            for b in bodies.into_iter() {
                                if min.x <= b.x && b.x < max.x && min.y <= b.y && b.y < max.y && min.z <= b.z && b.z < max.z {
                                    bodies_upper_south.push(b);
                                }
                            }
                            let result = Node::new_node(&mut bodies_upper_south, min, max);
                            match result {
                                Some(node) => self.children.push(node),
                                None => (),
                            }
                        }
                        (0, 0, 1) => {
                            for b in bodies.into_iter() {
                                if min.x <= b.x && b.x < max.x && min.y <= b.y && b.y < max.y && min.z <= b.z && b.z < max.z {
                                    bodies_upper_west.push(b);
                                }
                            }
                            let result = Node::new_node(&mut bodies_upper_west, min, max);
                            match result {
                                Some(node) => self.children.push(node),
                                None => (),
                            }
                        }
                        (1, 0, 1) => {
                            for b in bodies.into_iter() {
                                if min.x <= b.x && b.x < max.x && min.y <= b.y && b.y < max.y && min.z <= b.z && b.z < max.z {
                                    bodies_upper_north.push(b);
                                }
                            }
                            let result = Node::new_node(&mut bodies_upper_north, min, max);
                            match result {
                                Some(node) => self.children.push(node),
                                None => (),
                            }
                        }
                        _ => println!("invalid"),
                    }
                }
            }
        }

        for child in &mut self.children {
            // println!("{}", child);
            child.make_branches(bodies);
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "id = {}, is leaf = {}, min = {}, max = {}, com = {}", self.id, self.is_leaf, self.min, self.max, self.com)
    }
}

pub fn init_root(bodies: &mut Vec<&Body>) -> Option<Node> {
    let com = calc_com(&bodies);
    let mut r: Vec<Real> = vec![0.0; bodies.len()];

    for (b, ri) in bodies.iter().zip(r.iter_mut())
    {
        *ri = ((b.x - com.x).powi(2) + (b.y - com.y).powi(2) + (b.z - com.z).powi(2)).sqrt();
    }

    let max_r = 1.01 * r.iter().fold(Real::MIN, |ai, &bi| ai.max(bi));

    let min = &Vector { x: -max_r, y: -max_r, z: -max_r } + &com;
    let max = &Vector { x: max_r, y: max_r, z: max_r } + &com;
    Node::new_node(bodies, min, max)
}

pub fn tree_walk(body: &Body, i: usize, node: &Node, theta: Real) -> Vector {
    let mut a = EMPTY_VEC;
    let mut temp: Real = 0.0;
    let softening: Real = 0.0;
    let g: Real = 1.0;
    let x: Real = body.x - node.com.x;
    let y: Real = body.y - node.com.y;
    let z: Real = body.z - node.com.z;
    let r: Real = (x * x + y * y + z * z + softening * softening).sqrt();
    if (node.size / r < theta) || (node.is_leaf == true) {
        if node.id == body.id {
            a = EMPTY_VEC;
        } else {
            temp = g * body.m / r.powi(3);
            a.x = temp * x;
            a.y = temp * y;
            a.z = temp * z;
        }
    } else {
        for child in node.children.iter() {
            a = &a + &tree_walk(body, i, child, theta);
        }
    }
    return a;
}

pub fn calc_forces_tree(bodies: &mut Vec<Body>, root: &Node) {
    let theta = 0.5;
    for (i, b) in bodies.iter_mut().enumerate() {
        let a = tree_walk(b, i, root, theta);
        // println!("{}", a);
        b.ax = a.x;
        b.ay = a.y;
        b.az = a.z;
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_tree_init() {
        let a = Body { id: 1, m: 1.0, x: 0.0, y: 1.0, z: 0.0, vx: 0.0, vy: 0.0, vz: 0.0, ax: 0.0, ay: 0.0, az: 0.0, softening: 0.0 };
        let b = Body { id: 2, m: 1.0, x: 0.0, y: -1.0, z: 0.0, vx: 0.0, vy: 0.0, vz: 0.0, ax: 0.0, ay: 0.0, az: 0.0, softening: 0.0 };
        let c = Body { id: 3, m: 1.0, x: 0.1, y: -1.0, z: 0.0, vx: 0.0, vy: 0.0, vz: 0.0, ax: 0.0, ay: 0.0, az: 0.0, softening: 0.0 };
        let d = Body { id: 4, m: 1.0, x: 0.5, y: -1.0, z: 0.0, vx: 0.0, vy: 0.0, vz: 0.0, ax: 0.0, ay: 0.0, az: 0.0, softening: 0.0 };
        let mut bodies_p: Vec<&Body> = vec![&a, &b, &c, &d];
        let mut bodies: Vec<Body> = vec![a, b, c, d];
        let result = init_root(&mut bodies_p);
        match result {
            Some(mut root) => {
                root.make_branches(&mut bodies_p);
                calc_forces_tree(&mut bodies, &root)
            }
            None => return,
        }
    }
}