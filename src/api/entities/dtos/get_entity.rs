use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct GetEntityResponse {
    pub entity_id: String,
    pub components: serde_json::Value,
}
