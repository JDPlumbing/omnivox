#[derive(Debug, Clone)]
pub struct DegradationData {
    pub object_id: Uuid,
    pub corrosion: f64,
    pub fatigue: f64,
    pub thermal: f64,
    pub total_integrity: f64,
}
