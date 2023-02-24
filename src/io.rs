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
        file.write_f64::<LittleEndian>(f64::from(bi.x))?;
        file.write_f64::<LittleEndian>(f64::from(bi.y))?;
        file.write_f64::<LittleEndian>(f64::from(bi.z))?;
        file.write_f64::<LittleEndian>(f64::from(bi.vx))?;
        file.write_f64::<LittleEndian>(f64::from(bi.vy))?;
        file.write_f64::<LittleEndian>(f64::from(bi.vz))?;
        file.write_f64::<LittleEndian>(f64::from(bi.ax))?;
        file.write_f64::<LittleEndian>(f64::from(bi.ay))?;
        file.write_f64::<LittleEndian>(f64::from(bi.az))?;
    }
    Ok(())
}

pub fn read_csv(path: &str) -> Result<(Vec<Real>,Vec<Real>,Vec<Real>,Vec<Real>,Vec<Real>,Vec<Real>,Vec<Real>), Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_path(path)?;
    let au: Real = 1.5e11;
    let m_sol: Real = 2e30;
    let day: Real = 24.0 * 60.0 * 60.0;

    let mut masses = vec![];
    let mut x = vec![];
    let mut y = vec![];
    let mut z = vec![];
    let mut vx = vec![];
    let mut vy = vec![];
    let mut vz = vec![];

    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        masses.push(record[1].parse::<Real>().unwrap() * m_sol);
        x.push(record[2].parse::<Real>().unwrap() * au);
        y.push(record[3].parse::<Real>().unwrap() * au);
        z.push(record[4].parse::<Real>().unwrap() * au);
        vx.push(record[5].parse::<Real>().unwrap() * au / day);
        vy.push(record[6].parse::<Real>().unwrap() * au / day);
        vz.push(record[7].parse::<Real>().unwrap() * au / day);

    }
    return Ok((masses, x, y, z, vx, vy, vz));
}