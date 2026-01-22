use axum::{ Router};
use axum::routing::{ get, post, put, delete };
use crate::shared::AppState;



pub mod pages;
pub use pages::*;


pub fn pages_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list_pages).post(create_page))
        .route("/{slug}", get(get_page))
        .route("/id/{id}", put(update_page))
        .route("/{slug}", delete(delete_page))

}
