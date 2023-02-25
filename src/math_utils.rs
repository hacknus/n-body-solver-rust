use crate::body::Body;
use crate::body::Vector;
use crate::Real;

pub fn calc_direct_force(bodies: &mut [Body], g: &Real) {
    let a = bodies
        .iter()
        .map(|bi| {
            bodies
                .iter()
                .map(|bj| {
                    if bi.id != bj.id {
                        let r = Vector {
                            x: bj.x - bi.x,
                            y: bj.y - bi.y,
                            z: bj.z - bi.z,
                        };
                        r * *g * bj.m / r.norm().powi(3)
                    } else {
                        Vector {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        }
                    }
                })
                .sum::<Vector>()
        })
        .collect::<Vec<Vector>>();

    for (bi, acci) in bodies.iter_mut().zip(a.iter()) {
        bi.ax = acci.x;
        bi.ay = acci.y;
        bi.az = acci.z;
    }
}

pub fn leapfrog(bodies: &mut [Body], dt: &Real, g: &Real) {
    for bi in bodies.iter_mut() {
        bi.x += bi.vx * 0.5 * dt;
        bi.y += bi.vy * 0.5 * dt;
        bi.z += bi.vz * 0.5 * dt;
    }

    calc_direct_force(bodies, g);

    for bi in bodies.iter_mut() {
        bi.vx += bi.ax * dt;
        bi.vy += bi.ay * dt;
        bi.vz += bi.az * dt;
        bi.x += bi.vx * 0.5 * dt;
        bi.y += bi.vy * 0.5 * dt;
        bi.z += bi.vz * 0.5 * dt;
    }
}

pub fn get_dt(bodies: &Vec<Body>) -> Real {
    let mut dt: Vec<Real> = vec![0.0; bodies.len()];
    let softening: Real = 0.01;
    let min_dt: Real;
    let mut a_mag: Real;
    for (i, bi) in bodies.iter().enumerate() {
        a_mag = (bi.ax * bi.ax + bi.ay * bi.ay + bi.az * bi.az).sqrt();
        dt[i] = (softening / a_mag).sqrt();
    }
    min_dt = dt.iter().fold(Real::INFINITY, |ai, &bi| ai.min(bi));
    // println!("min_dt is {:.32}", min_dt);
    return min_dt;
}
