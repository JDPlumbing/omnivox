use crate::supabasic::Supabase;
use crate::supabasic::worlds::WorldRow;
use crate::supabasic::entity::EntityRow;

use crate::core::id::WorldId;
use crate::core::tdt::sim_time::SimTime;

use crate::engine::entities::SimEntity;
use crate::engine::world::state::{World, WorldState};

use anyhow::Result;

/// ---------------------------------------------------------------------------
/// Load a runtime WorldState from Supabase by typed WorldId.
/// ---------------------------------------------------------------------------
pub async fn load_world(
    supa: &Supabase,
    world_id: WorldId,
) -> Result<WorldState> {

    //
    // 1. Load world metadata row
    //
    let meta_rec = WorldRow::get(supa, world_id).await?;

    //
    // Convert DB → runtime metadata World
    //
    let meta = World {
        id: world_id,
        name: meta_rec.name.clone(),
        description: meta_rec.description.clone(),
        world_epoch: meta_rec.world_epoch
            .as_ref()
            .and_then(|s| s.parse::<i128>().ok())
            .map(SimTime::from_ns),

    };

    //
    // 2. Load all entity records for this world
    //
    let rows: Vec<EntityRow> =
        EntityRow::list_for_world(supa, world_id).await?;

    //
    // 3. Convert each DB row into a SimEntity
    //
    let mut state = WorldState::new(meta);

    for row in rows {
        match SimEntity::try_from(row) {
            Ok(ent) => {
                // ✔ Insert using EntityId instead of Uuid
                state.entities.insert(ent.id, ent);
            }
            Err(e) => {
                eprintln!("⚠ Failed to convert EntityRow → SimEntity: {:?}", e);
            }
        }
    }

    //
    // 4. Return hydrated runtime state
    //
    Ok(state)
}
