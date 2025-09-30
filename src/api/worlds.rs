use axum::{
    extract::Path,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::sim::world::{SimWorld, SimWorldDto};

pub async fn get_world(Path(world_id): Path<Uuid>) -> impl IntoResponse {
    // For now just return a default world
    // Later: replace with SimWorld::load(&supa, world_id).await
    let world = SimWorld::default();

    let dto: SimWorldDto = (&world).into();
    Json(dto)
}
