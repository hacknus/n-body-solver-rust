use crate::body::Body;
use crate::body::Acc;
use crate::body::EMPTY_ACC;
use crate::Real;

pub fn calc_direct_force(
    masses: &Vec<Real>,
    x: &mut Vec<Real>,
    y: &mut Vec<Real>,
    z: &mut Vec<Real>,
    ax: &mut Vec<Real>,
    ay: &mut Vec<Real>,
    az: &mut Vec<Real>,
) {
    let g: Real = 6.67408e-11;
    let mut rxi: Real;
    let mut ryi: Real;
    let mut rzi: Real;
    let mut r: Real;
    let mut temp: Real;

    for i in 0..x.len() {
        for j in 0..x.len() {
            if i != j {
                rxi = x[j] - x[i];
                ryi = y[j] - y[i];
                rzi = z[j] - z[i];
                r = (rxi * rxi + ryi * ryi + rzi * rzi).sqrt();
                temp = g * masses[j] / r.powi(3);
                ax[i] += temp * rxi;
                ay[i] += temp * ryi;
                az[i] += temp * rzi;
            }
        }
    }
}

pub fn leapfrog(
    masses: &Vec<Real>,
    x: &mut Vec<Real>,
    y: &mut Vec<Real>,
    z: &mut Vec<Real>,
    vx: &mut Vec<Real>,
    vy: &mut Vec<Real>,
    vz: &mut Vec<Real>,
    ax: &mut Vec<Real>,
    ay: &mut Vec<Real>,
    az: &mut Vec<Real>,
    dt: Real) {
    for (xi, (yi, (zi, (vxi, (vyi, vzi))))) in x.iter_mut().zip(y.iter_mut().zip(z.iter_mut().zip(vx.iter().zip(vy.iter().zip(vz.iter()))))) {
        *xi += *vxi * 0.5 * dt;
        *yi += *vyi * 0.5 * dt;
        *zi += *vzi * 0.5 * dt;
    }

    calc_direct_force(masses, x, y, z, ax, ay, az);

    for (xi, (yi, (zi, (vxi, (vyi, (vzi, (axi, (ayi, (azi))))))))) in x.iter_mut().zip(y.iter_mut().zip(z.iter_mut().zip(vx.iter_mut().zip(vy.iter_mut().zip(vz.iter_mut().zip(ax.iter().zip(ay.iter().zip(az.iter())))))))) {
        *vxi += axi * dt;
        *vyi += ayi * dt;
        *vzi += azi * dt;
        *xi += *vxi * 0.5 * dt;
        *yi += *vyi * 0.5 * dt;
        *zi += *vzi * 0.5 * dt;
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