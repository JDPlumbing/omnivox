pub use crate::supabasic::orm::{DbModel, list, insert};
pub use crate::sim::world::{World, NewWorld};

impl DbModel for World {
    fn table() -> &'static str { "worlds" }
}

impl DbModel for NewWorld {
    fn table() -> &'static str { "worlds" }
}

// Convenience wrappers
pub async fn list_worlds() -> Result<Vec<World>, crate::supabasic::SupabasicError> {
    list::<World>().await
}

pub async fn create_world(payload: &NewWorld) -> Result<World, crate::supabasic::SupabasicError> {
    insert::<NewWorld, World>(payload).await
}
