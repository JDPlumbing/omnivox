use axum::{Router, routing::{get, post, put, delete}};
use crate::shared::app_state::AppState;

pub mod address;
pub mod geocode;
pub mod uvox;

pub fn location_routes() -> Router<AppState> {
    Router::new()
        // Address CRUD
        .route("/address", 
            get(address::list_addresses)
                .post(address::create_address)
        )
        .route("/address/{id}",
            get(address::get_address)
                .put(address::update_address)
                .delete(address::delete_address)
        )
        // Address Resolve
        .route("/address/{id}/resolve", 
            post(geocode::resolve_address)
        )

        // Uvox converters
        .route("/uvox/from_coords", post(uvox::coords_to_uvox))
        .route("/uvox/to_coords", post(uvox::uvox_to_coords))
}
