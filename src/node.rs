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
    pub size: Real,
    pub is_leaf: bool,
}

pub const EMPTY_NODE: Node = Node {
    id: 0,
    children: vec![],
    m: 0.0,
    com: EMPTY_VEC,
    size: 0.0,
    is_leaf: false,
};

impl Node {
    pub fn new_node(mut bodies: &mut Vec<&Body>, min: &Vector, max: &Vector) -> Option<Node> {
        let mut is_leaf = false;
        let mut id = 0;

        if bodies.len() > 0 {
            let size = (max - min).norm();
            let m_tot = calc_m_tot(&bodies);
            let com = calc_com(&bodies);
            if bodies.len() == 1 {
                id = bodies[0].id;
                is_leaf = true;
            }
            let mut new_node = Node { id, children: Vec::new(), m: m_tot, com, size, is_leaf };
            new_node.make_branches(bodies, min, max);
            Some(new_node)
        } else {
            None
        }
    }


    pub fn make_branches(&mut self, mut bodies: &mut Vec<&Body>, my_min: &Vector, my_max: &Vector) {
        if self.is_leaf == true {
            return;
        }
        let width = (my_max.x - my_min.x) * 0.5;
        let offset = Vector { x: width, y: width, z: width };
        let mut bodies_children: [[[Vec<&Body>; 2]; 2]; 2] = [[[vec![], vec![]], [vec![], vec![]]], [[vec![], vec![]], [vec![], vec![]]]];
        let mut min_children: [[[Vector; 2]; 2]; 2] = [[[EMPTY_VEC, EMPTY_VEC], [EMPTY_VEC, EMPTY_VEC]], [[EMPTY_VEC, EMPTY_VEC], [EMPTY_VEC, EMPTY_VEC]]];
        let mut max_children: [[[Vector; 2]; 2]; 2] = [[[EMPTY_VEC, EMPTY_VEC], [EMPTY_VEC, EMPTY_VEC]], [[EMPTY_VEC, EMPTY_VEC], [EMPTY_VEC, EMPTY_VEC]]];
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let mut min = my_min + &Vector { x: i as f32 * width, y: j as f32 * width, z: k as f32 * width };
                    let mut max = my_min + &offset;
                    max = &max + &Vector { x: i as f32 * width, y: j as f32 * width, z: k as f32 * width };
                    min_children[i][j][k] = min;
                    max_children[i][j][k] = max;
                }
            }
        }
        for b in bodies.into_iter() {
            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        if min_children[i][j][k].x <= b.x && b.x < max_children[i][j][k].x && min_children[i][j][k].y <= b.y && b.y < max_children[i][j][k].y && min_children[i][j][k].z <= b.z && b.z < max_children[i][j][k].z {
                            bodies_children[i][j][k].push(b);
                        }
                    }
                }
            }
        }
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let result = Node::new_node(&mut bodies_children[i][j][k], &min_children[i][j][k], &max_children[i][j][k]);
                    match result {
                        Some(node) => self.children.push(node),
                        None => (),
                    }
                }
            }
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "id = {}, is leaf = {}, com = {}", self.id, self.is_leaf, self.com)
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
    Node::new_node(bodies, &min, &max)
}