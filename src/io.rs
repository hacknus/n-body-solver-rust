use crate::body::Body;
use std::error::Error;
use std::fs::File;
use byteorder::WriteBytesExt;
// This trait adds methods to writeable types
use byteorder::LittleEndian;
use crate::Real;

pub fn write_file(path: &str, bodies: &Vec<Body>) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    for bi in bodies.iter(){
        file.write_f64::<LittleEndian>(bi.x as f64)?;
        file.write_f64::<LittleEndian>(bi.y as f64)?;
        file.write_f64::<LittleEndian>(bi.z as f64)?;
        file.write_f64::<LittleEndian>(bi.vx as f64)?;
        file.write_f64::<LittleEndian>(bi.vy as f64)?;
        file.write_f64::<LittleEndian>(bi.vz as f64)?;
        file.write_f64::<LittleEndian>(bi.ax as f64)?;
        file.write_f64::<LittleEndian>(bi.ay as f64)?;
        file.write_f64::<LittleEndian>(bi.az as f64)?;
    }
    Ok(())
}

pub fn read_csv(path: &str) -> Result<Vec<Body>, Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_path(path)?;
    let mut bodies: Vec<Body> = Vec::new();
    let au: Real = 1.5e11;
    let m_sol: Real = 2e30;
    let day: Real = 24.0 * 60.0 * 60.0;
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        let new_body = Body {
            m: record[1].parse::<Real>().unwrap() * m_sol,
            x: record[2].parse::<Real>().unwrap() * au,
            y: record[3].parse::<Real>().unwrap() * au,
            z: record[4].parse::<Real>().unwrap() * au,
            vx: record[5].parse::<Real>().unwrap() * au / day,
            vy: record[6].parse::<Real>().unwrap() * au / day,
            vz: record[7].parse::<Real>().unwrap() * au / day,
            ax: 0.0,
            ay: 0.0,
            az: 0.0,
            softening: 0.001,
        };
        bodies.push(new_body);
    }
    return Ok(bodies);
}