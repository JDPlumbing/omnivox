use axum::{
    extract::{Path, State, Json, Extension},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use uuid::Uuid;

use crate::shared::app_state::AppState;
use crate::shared::identity::auth_context::AuthContext;
use crate::core::WorldId;
use crate::core::property::property_update::UpdateProperty;
use crate::api::properties::payloads::property_input::PropertyInput;

pub async fn update_property(
    Extension(auth): Extension<AuthContext>,
    State(state): State<AppState>,
    Path((world_id, property_id)): Path<(WorldId, Uuid)>,
    Json(input): Json<PropertyInput>,
) -> impl IntoResponse {
    let cmd = UpdateProperty {
        property_id,
        actor_id: auth.user_id,
        world_id,

        address_id: input.address_id,
        name: input.name,
        anchor_uvox: Some(input.anchor.uvox),

        square_feet: input.square_feet,
        bedrooms: input.bedrooms,
        bathrooms: input.bathrooms,
    };

    match state.property_source.update(cmd).await {
        Ok(property) => Json(property).into_response(),

        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}
