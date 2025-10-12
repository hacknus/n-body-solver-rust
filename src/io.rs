use crate::body::Body;
use std::error::Error;
use std::fs::File;
use byteorder::WriteBytesExt;
// This trait adds methods to writeable types
use byteorder::LittleEndian;
use crate::Real;

pub fn write_file(path: &str, bodies: &Vec<Body>) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    for bi in bodies.iter() {
        file.write_f64::<LittleEndian>(bi.x as f64)?;
        file.write_f64::<LittleEndian>(bi.y as f64)?;
        file.write_f64::<LittleEndian>(bi.z as f64)?;
        file.write_f64::<LittleEndian>(bi.vx as f64)?;
        file.write_f64::<LittleEndian>(bi.vy as f64)?;
        file.write_f64::<LittleEndian>(bi.vz as f64)?;
        file.write_f64::<LittleEndian>(bi.ax as f64)?;
        file.write_f64::<LittleEndian>(bi.ay as f64)?;
        file.write_f64::<LittleEndian>(bi.az as f64)?;
        file.write_f64::<LittleEndian>(bi.radius as f64)?;
        file.write_f64::<LittleEndian>(if bi.escaped { 1.0 } else { 0.0 })?;  // Write escaped flag
    }
    Ok(())
}



pub fn read_csv(path: &str) -> Result<Vec<Body>, Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_path(path)?;
    let mut bodies: Vec<Body> = Vec::new();

    for result in rdr.records() {
        let record = result?;

        // Parse radius directly from CSV (assuming it's in record[8])
        let radius = record[8].parse::<Real>().unwrap();

        let new_body = Body {
            m: record[1].parse::<Real>().unwrap(),                // Mass in kg
            x: record[2].parse::<Real>().unwrap(),                // Position in meters
            y: record[3].parse::<Real>().unwrap(),
            z: record[4].parse::<Real>().unwrap(),
            vx: record[5].parse::<Real>().unwrap(),               // Velocity in m/s
            vy: record[6].parse::<Real>().unwrap(),
            vz: record[7].parse::<Real>().unwrap(),
            ax: 0.0,
            ay: 0.0,
            az: 0.0,
            radius: radius,                                       // Radius in meters
            softening: 4.0 * radius,                              // Softening = 2 * radius
            agglomerate_id: -1,                                   // -1 for free particles
            escaped: false,
        };
        bodies.push(new_body);
    }
    return Ok(bodies);
}
