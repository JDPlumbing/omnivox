use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct FractureData {
    pub object_id: Uuid,
    pub plane: String,
    pub energy: f64,
    pub threshold: f32,
}
