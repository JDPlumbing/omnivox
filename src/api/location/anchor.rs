use axum::{
    extract::{State, Json},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use anyhow::anyhow;
use serde::Deserialize;
use uuid::Uuid;

use crate::shared::app_state::AppState;
use crate::core::spatial::CreateAddress;
use crate::supabasic::addresses::AddressRow;

#[derive(Deserialize)]
pub struct ResolveAddressAnchorReq {
    pub street_address: String,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
}

pub async fn resolve_address_anchor(
    State(app): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<ResolveAddressAnchorReq>,
) -> impl IntoResponse {

    // 1️⃣ Extract session
    let Some(session_id) = headers
        .get("x-session-id")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| Uuid::parse_str(s).ok())
    else {
        return StatusCode::UNAUTHORIZED.into_response();
    };

    // 2️⃣ Load session
    let session = match app.session_source.get_session(session_id).await {
        Ok(Some(s)) => s,
        _ => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let world_id = match session.world_id {
        Some(w) => w,
        None => return StatusCode::BAD_REQUEST.into_response(),
    };

    // 3️⃣ Create address (infra)
    let address = match app.address_source
        .create(AddressRow {
            id: None,
            street_address: Some(req.street_address),
            city: req.city,
            state: req.state,
            postal_code: req.postal_code,
            country: req.country.or(Some("us".into())),
        })
        .await
    {
        Ok(a) => a,
        Err(e) => return (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    };

    let address_id = address.id
        .ok_or_else(|| anyhow!("Address missing id"))
        .unwrap();

    // 4️⃣ Resolve anchor (ENGINE)
    let anchor = match app.location_engine
        .anchor_from_address(world_id, address_id)
        .await
    {
        Ok(a) => a,
        Err(e) => return (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    };

    // 5️⃣ Attach to session
    if let Err(e) = app.session_source
        .set_spatial_anchor(session_id, anchor.clone())
        .await
    {
        return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
    }

    Json(anchor).into_response()
}
