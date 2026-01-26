use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateUserPayload {
    pub email: String,
    pub password: String,
    pub display_name: String,
    pub role: String,
}
