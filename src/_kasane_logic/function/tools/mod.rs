pub mod ecef_to_id;
pub mod ecef_to_point;
pub mod point_to_ecef;
pub mod point_to_id;

#[derive(Debug, Clone, Copy)]
pub struct ECEF {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
