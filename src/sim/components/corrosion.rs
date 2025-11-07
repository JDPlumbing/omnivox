use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CorrosionData {
    pub object_id: Uuid,
    pub rate: f64,
    pub severity: f32,
}   