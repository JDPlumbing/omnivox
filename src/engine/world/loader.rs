use crate::supabasic::Supabase;
use crate::supabasic::worlds::WorldRow;
use crate::supabasic::entity::EntityRow;

use crate::core::id::WorldId;
use crate::core::tdt::sim_time::SimTime;

use crate::core::SimEntity;
use crate::engine::world::state::WorldState;
use crate::core::world::World;

use anyhow::Result;
use crate::core::world::world_env_descriptor::WorldEnvDescriptor;
use crate::core::world::WorldEnvironment;
use crate::core::world::presets::earth_v0;


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
    let env_desc = match &meta_rec.environment {
    Some(desc) => desc.clone(),
    None => {
        log::warn!(
            "World {} has no environment, defaulting to earth_v0",
            world_id
        );
        earth_v0()
    }
};

let world_env = WorldEnvironment::from_descriptor(&env_desc);
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
    let mut state = WorldState::new(meta, world_env);

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
