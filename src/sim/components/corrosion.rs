use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CorrosionData {
    pub object_id: Uuid,
    pub surface_area: f64,     // m², to localize corrosion loss
    pub thickness_loss: f64,   // m, material lost over time
    pub rate: f64,             // m/s (corrosion rate)
    pub environment_factor: f32, // humidity, salinity, acidity, etc.
    pub severity: f32,         // 0.0–1.0, normalized damage
}
