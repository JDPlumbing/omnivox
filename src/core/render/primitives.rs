use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RenderPrimitive {
    Point {
        position: [f64; 3],
        size: f64,
    },

    Sphere {
        center: [f64; 3],
        radius: f64,
        cosmic_body_id: i64,
    },

    Line {
        from: [f64; 3],
        to: [f64; 3],
    },

    Vector {
        origin: [f64; 3],
        direction: [f64; 3],
        scale: f64,
    },

    /// Omnidirectional emitter (lamps, stars at small scale)
    PointLight {
        position: [f64; 3],
        intensity: f64,
    },

    /// Directional light (sunlight)
    DirectionalLight {
        direction: [f64; 3],
        intensity: f64,
    },
}
