use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use crate::shared::app_state::AppState;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Page {
    pub id: Option<uuid::Uuid>,
    pub slug: String,
    pub title: String,
    pub content: serde_json::Value,
    pub media_url: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

// GET /api/pages/:slug
pub async fn get_page(
    Path(slug): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<Page>, StatusCode> {
    let result = state
        .supa
        .from("pages")
        .select("*")
        .eq("slug", &slug)
        .single()
        .await;

    match result {
        Ok(value) => {
            let page: Page = serde_json::from_value(value)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(Json(page))
        }
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}
/// GET /api/pages — list all pages
pub async fn list_pages(State(state): State<AppState>) -> Result<Json<Vec<Page>>, StatusCode> {
    let result = state
        .supa
        .from("pages")
        .select("*")
        .execute()
        .await;

    match result {
        Ok(data) => {
            let pages: Vec<Page> = serde_json::from_value(data).unwrap_or_default();
            Ok(Json(pages))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// POST /api/pages  

pub async fn create_page(
    State(state): State<AppState>,
    Json(mut payload): Json<Page>,
) -> Result<Json<Page>, StatusCode> {
    // If the client didn't send an ID, generate one manually
    if payload.id.is_none() {
        payload.id = Some(Uuid::new_v4());
    }

    let result = state
        .supa
        .from("pages")
        .insert(serde_json::json!(payload))
        .execute()
        .await;

    match result {
        Ok(value) => {
            // Handle array or single object
            let page: Page = if let Some(arr) = value.as_array() {
                serde_json::from_value(arr[0].clone())
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            } else {
                serde_json::from_value(value)
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            };

            Ok(Json(page))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }

}

// PUT /api/pages/:slug
pub async fn update_page(
    Path(id): Path<uuid::Uuid>,
    State(state): State<AppState>,
    Json(payload): Json<Page>,
) -> Result<Json<Page>, StatusCode> {
    let mut page_json = serde_json::to_value(&payload)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    if let Some(obj) = page_json.as_object_mut() {
        obj.remove("id");
    }

    let result = state
        .supa
        .from("pages")
        .update(page_json)
        .eq("id", &id.to_string()) // ✅ match by ID
        .execute()
        .await;

    match result {
        Ok(value) => {
            let updated_page: Option<Page> = value
                .as_array()
                .and_then(|arr| arr.first())
                .and_then(|first| serde_json::from_value(first.clone()).ok());

            if let Some(page) = updated_page {
                Ok(Json(page))
            } else {
                Err(StatusCode::NOT_FOUND)
            }
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// DELETE /api/pages/:slug
pub async fn delete_page(
    Path(slug): Path<String>,
    State(state): State<AppState>,
) -> Result<StatusCode, StatusCode> {
    let result = state
        .supa
        .from("pages")
        .delete()
        .eq("slug", &slug)
        .execute()
        .await;

    match result {
        Ok(res) => {
            eprintln!("✅ Deleted page: {:?}", res);
            Ok(StatusCode::NO_CONTENT)
        }
        Err(err) => {
            eprintln!("❌ Failed to delete page: {:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
