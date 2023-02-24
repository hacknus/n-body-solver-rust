mod body;
mod io;
mod math_utils;

use std::env;
use std::time::Instant;

use crate::body::Body;
use crate::io::{read_csv, write_file};
use crate::math_utils::{calc_direct_force, get_dt, leapfrog};

type Real = f64;

fn mean(data: &[f32]) -> Option<f32> {
    let sum = data.iter().sum::<f32>() as f32;
    let count = data.len();

    match count {
        positive if positive > 0 => Some(sum / count as f32),
        _ => None,
    }
}

fn std(data: &[f32]) -> Option<f32> {
    match (mean(data), data.len()) {
        (Some(data_mean), count) if count > 0 => {
            let variance = data
                .iter()
                .map(|value| {
                    let diff = data_mean - (*value as f32);

                    diff * diff
                })
                .sum::<f32>()
                / count as f32;

            Some(variance.sqrt())
        }
        _ => None,
    }
}

fn main() {
    println!("Let's calculate some orbits! ");
    let init_start = Instant::now();

    let path = "solar_jfc.dat";
    let steps = 100;
    let mut bodies: Vec<Body>;

    bodies = match read_csv(path) {
        Err(e) => panic!("Problem opening the file: {:?}", e),
        Ok(b) => b,
    };

    let dt = 60.0 * 60.0 * 24.0;
    let g: Real = 6.67408e-11;

    println!("init time: {:?}", init_start.elapsed());

    println!("starting calculation...");
    let mut times = vec![];
    for i in 0..2198 {
        let start = Instant::now();
        // calculate first forces, in order to get initial dt
        // calc_direct_force(&mut bodies);
        for step in 0..steps {
            // dt = get_dt(&bodies);
            // dt = 60.0 * 60.0 * 24.0;
            // t += dt;
            leapfrog(&mut bodies, &dt, &g);
            // println!("calculating step {} at time t+{:.5}", step, t);
            // if step % 10 == 0 {
            //     match write_file(&format!("output/out{:0>5}.dat", step), &bodies) {
            //         Err(e) => panic!("Problem writing the output file: {:?}", e),
            //         Ok(()) => (),
            //     }
            // }
        }
        let time_passed = start.elapsed();
        times.push(time_passed.as_micros() as f32 / 1000.0);
    }
    write_file("jura_test.bin", &bodies).expect("failed to save file");
    println!(
        "runtime: {:.4}ms +/- {:.4}ms",
        mean(&times).unwrap(),
        std(&times).unwrap()
    );
}
