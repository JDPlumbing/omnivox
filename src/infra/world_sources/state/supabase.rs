use anyhow::Result;
use async_trait::async_trait;

use crate::supabasic::Supabase;
use crate::supabasic::worlds::WorldRow;
use crate::supabasic::entity::EntityRow;

use crate::core::id::{WorldId, EntityId};
use crate::core::tdt::sim_time::SimTime;

use crate::core::world::{World, WorldEnvironment};
use crate::core::world::presets::earth_v0;

use crate::core::components::{
    position::Position,
    orientation::Orientation,
    world_membership::WorldMembership,
    spawned_at::SpawnedAt,
};

use crate::engine::world::state::WorldState;
use crate::engine::entity::EntityEngine;

use crate::shared::world_sources::state::source::WorldStateSource;

use crate::core::UvoxId;

pub struct SupabaseWorldStateSource {
    supa: Supabase,
}

impl SupabaseWorldStateSource {
    pub fn new(supa: Supabase) -> Self {
        Self { supa }
    }
}

#[async_trait]
impl WorldStateSource for SupabaseWorldStateSource {
    async fn load_world_state(
        &self,
        world_id: WorldId,
    ) -> Result<WorldState> {

        // --- Load world metadata ---
        let meta_rec = WorldRow::get(&self.supa, world_id).await?;

        let env_desc = meta_rec
            .environment
            .clone()
            .unwrap_or_else(|| {
                log::warn!(
                    "World {} has no environment, defaulting to earth_v0",
                    world_id
                );
                earth_v0()
            });

        let world_env = WorldEnvironment::from_descriptor(&env_desc);

        let meta = World {
            id: world_id,
            name: meta_rec.name.clone(),
            description: meta_rec.description.clone(),
            world_epoch: meta_rec
                .world_epoch
                .as_ref()
                .and_then(|s| s.parse::<i128>().ok())
                .map(SimTime::from_ns),
        };

        let mut state = WorldState::new(meta, world_env);
        let mut entity_engine = EntityEngine::new(&mut state.entity_store);

        // --- Load entities + components ---
        let rows: Vec<EntityRow> =
            EntityRow::list_for_world(&self.supa, world_id).await?;

        for row in rows {
            let entity_id = EntityId(
                row.row_id.expect("EntityRow missing row_id"),
            );

            entity_engine.set_world(
                entity_id,
                WorldMembership { world_id },
            );

            if let Some(spawned_ns) = row.spawned_at {
                entity_engine.set_spawned_at(
                    entity_id,
                    SpawnedAt {
                        time: SimTime::from_ns(spawned_ns),
                    },
                );
            }

            let uvox: UvoxId =
                serde_json::from_value(row.position)
                    .expect("Invalid UvoxId in DB");

            entity_engine.set_position(
                entity_id,
                Position(uvox),
            );

            if !row.orientation.is_null() {
                let orientation =
                    serde_json::from_value(row.orientation)
                        .expect("Invalid orientation in DB");

                entity_engine.set_orientation(
                    entity_id,
                    Orientation(orientation),
                );
            }

            if !row.metadata.is_null() {
                entity_engine.set_metadata(
                    entity_id,
                    row.metadata,
                );
            }
        }

        Ok(state)
    }
}
