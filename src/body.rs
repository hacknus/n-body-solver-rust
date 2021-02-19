#[derive(PartialEq, PartialOrd)]
pub struct Body {
    pub m: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub vx: f64,
    pub vy: f64,
    pub vz: f64,
    pub ax: f64,
    pub ay: f64,
    pub az: f64,
    pub softening: f64,
}

// impl Default for Body {
//     fn default() -> Body {
//         Body {
//             m: 1.0,
//             x: 0.0,
//             y: 0.0,
//             z: 0.0,
//             vx: 0.0,
//             vy: 0.0,
//             vz: 0.0,
//             ax: 0.0,
//             ay: 0.0,
//             az: 0.0,
//             softening: 0.0,
//         }
//     }
// }