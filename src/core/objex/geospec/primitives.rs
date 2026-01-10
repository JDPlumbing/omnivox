use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Line {
    pub length: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plane {
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sphere {
    pub radius: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoxShape {
    pub length: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cylinder {
    pub radius: f64,
    pub height: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cone {
    pub radius: f64,
    pub height: f64,
}
