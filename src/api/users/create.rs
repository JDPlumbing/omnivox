use serde::Deserialize;
use serde_json::json;
use reqwest::Client;
use std::env;

use crate::shared::app_state::AppState;
use crate::supabasic::users::User;

#[derive(Debug, Deserialize)]
pub struct CreateUserPayload {
    pub email: String,
    pub password: String,
    pub display_name: String,
    pub role: String,
}

pub async fn create_user_service(
    app: &AppState,
    payload: CreateUserPayload,
) -> Result<User, anyhow::Error> {

    let supabase_url =
        env::var("SUPABASE_URL").expect("SUPABASE_URL not set");
    let service_role_key =
        env::var("SUPABASE_SERVICE_ROLE_KEY").expect("SUPABASE_SERVICE_ROLE_KEY not set");

    let client = Client::new();

    // 1️⃣ Create auth user
    let auth_res = client
        .post(format!("{}/auth/v1/admin/users", supabase_url))
        .header("Authorization", format!("Bearer {}", service_role_key))
        .header("apikey", &service_role_key)
        .json(&json!({
            "email": payload.email,
            "password": payload.password,
            "email_confirm": true,
            "user_metadata": {
                "display_name": payload.display_name,
                "role": payload.role
            }
        }))
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    eprintln!("🧠 Supabase auth response: {:?}", auth_res);

    let auth_id = auth_res["id"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Auth user creation failed"))?;

    // 2️⃣ Fetch domain user created by trigger
    let user = app
        .supa
        .from("users")
        .select("*")
        .eq("id", auth_id)
        .single_typed::<User>()
        .await?;

    Ok(user)
}
