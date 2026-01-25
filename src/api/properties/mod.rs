pub mod properties;

pub use properties::*;


use axum::{
    routing::{get, post, put, patch, delete},
    Router,
};

    pub fn property_routes() -> Router {
        Router::new()
        .route("/", get(list_properties).post(create_property))
        .route(
            "/{id}",
            get(get_property)
                .put(update_property)
                .delete(delete_property),
        )
        .route("/world/{world_id}", get(list_properties_for_world))
        ;
    }