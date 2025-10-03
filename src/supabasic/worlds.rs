use crate::supabasic::client::Supabase;
use crate::supabasic::SupabasicError;
use crate::sim::world::{World, NewWorld};

/// List all worlds
pub async fn list_worlds() -> Result<Vec<World>, SupabasicError> {
    let supa = Supabase::new_from_env()?;

    let worlds: Vec<World> = supa
        .from("worlds")
        .select("frame_id,name,description,created_at,updated_at,deleted_at")
        .execute_typed()
        .await?;

    Ok(worlds)
}

/// Get a single world by its frame_id
pub async fn get_world_by_frame_id(frame_id: i64) -> Result<World, SupabasicError> {
    let supa = Supabase::new_from_env()?;

    let world: World = supa
        .from("worlds")
        .select("frame_id,name,description,created_at,updated_at,deleted_at")
        .eq("frame_id", &frame_id.to_string())
        .single_typed()
        .await?;

    Ok(world)
}

/// Create a new world
pub async fn create_world(payload: &NewWorld) -> Result<World, SupabasicError> {
    let supa = Supabase::new_from_env()?;

    let inserted: Vec<World> = supa
        .from("worlds")
        .insert(payload)
        .select("frame_id,name,description,created_at,updated_at,deleted_at")
        .execute_typed()
        .await?;

    inserted
        .into_iter()
        .next()
        .ok_or_else(|| SupabasicError::Other("empty insert response".into()))
}
