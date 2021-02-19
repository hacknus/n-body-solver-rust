#[path = "body.rs"]
mod body;

use crate::body::Body;
use std::error::Error;

pub fn read_csv(path: &str) -> Result<Vec<Body>, Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_path(path)?;
    let mut bodies: Vec<Body> = Vec::new();
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        bodies.push(Body {
            m: record[1].parse::<f64>().unwrap(),
            x: record[2].parse::<f64>().unwrap(),
            y: record[3].parse::<f64>().unwrap(),
            z: record[4].parse::<f64>().unwrap(),
            vx: record[5].parse::<f64>().unwrap(),
            vy: record[6].parse::<f64>().unwrap(),
            vz: record[7].parse::<f64>().unwrap(),
            ax: 0.0,
            ay: 0.0,
            az: 0.0,
            softening: 0.0,
        });
    }
    return Ok(bodies);
}