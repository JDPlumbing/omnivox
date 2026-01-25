use serde::Deserialize; 
use serde_json::Value;

#[derive(Deserialize)]
pub struct CreateWorldPayload {
    pub name: Option<String>,
    pub description: Option<String>,
    pub environment: Option<Value>,
}
