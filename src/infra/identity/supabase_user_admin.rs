// infra/identity/supabase_user_admin.rs
use serde_json::json;
use reqwest::Client;
use std::env;

use crate::supabasic::users::User;
use crate::supabasic::Supabase;

pub struct NewUser {
    pub email: String,
    pub password: String,
    pub display_name: String,
    pub role: String,
}

pub async fn provision_user(
    new_user: NewUser,
) -> Result<User, anyhow::Error> {
    let supabase_url = env::var("SUPABASE_URL")?;
    let service_role_key = env::var("SUPABASE_SERVICE_ROLE_KEY")?;

    let NewUser {
        email,
        password,
        display_name,
        role,
    } = new_user;

    let client = Client::new();

    let auth_res = client
        .post(format!("{}/auth/v1/admin/users", supabase_url))
        .header("Authorization", format!("Bearer {}", service_role_key))
        .header("apikey", &service_role_key)
        .json(&json!({
            "email": email,
            "password": password,
            "email_confirm": true,
            "user_metadata": {
                "display_name": display_name,
                "role": role
            }
        }))
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    let auth_id = auth_res["id"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Auth user creation failed"))?;

    let supa = Supabase::new_from_env()?;

    let user = supa
        .from("users")
        .select("*")
        .eq("id", auth_id)
        .single_typed::<User>()
        .await?;

    Ok(user)
}
