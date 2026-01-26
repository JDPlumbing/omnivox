use axum::{
    extract::{State, Path, Json, Extension},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;

use crate::shared::app_state::AppState;
use crate::shared::identity::auth_context::AuthContext;
use crate::core::WorldId;
use crate::api::properties::payloads::property_input::PropertyInput;
use crate::core::CreateProperty;

pub async fn create_property(
    Extension(auth): Extension<AuthContext>,
    State(state): State<AppState>,
    Path(world_id): Path<WorldId>,
    Json(input): Json<PropertyInput>,
) -> impl IntoResponse {
    let cmd = CreateProperty {
        owner_id: auth.user_id,
        world_id,

        address_id: input.address_id,
        name: input.name,
        anchor_uvox: input.anchor.uvox,

        square_feet: input.square_feet,
        bedrooms: input.bedrooms,
        bathrooms: input.bathrooms,
    };

    match state.property_source.create(cmd).await {
        Ok(property) => (
            StatusCode::CREATED,
            Json(property),
        )
            .into_response(),

        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}
