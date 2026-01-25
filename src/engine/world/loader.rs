use crate::supabasic::Supabase;
use crate::supabasic::worlds::WorldRow;
use crate::supabasic::entity::EntityRow;

use crate::core::id::{WorldId, EntityId};
use crate::core::tdt::sim_time::SimTime;
use crate::core::{ UvoxId};


use crate::engine::world::state::{
    WorldState,
};
use crate::core::components::lifecycle::Lifecycle;
use crate::core::world::WorldEnvironment;
use crate::core::components::position::Position;
use crate::core::components::orientation::Orientation;

use crate::core::world::World;

use crate::core::world::presets::earth_v0;

use anyhow::Result;
// TODO(world-sources):
// Extract world loading into shared::world_sources::WorldSource
// Implement SupabaseWorldSource and JsonWorldSource
// WorldState must be loadable without DB access


/// ---------------------------------------------------------------------------
/// Load a runtime ECS WorldState from Supabase by typed WorldId.
/// ---------------------------------------------------------------------------
pub async fn load_world(
    supa: &Supabase,
    world_id: WorldId,
) -> Result<WorldState> {

    // 1. Load world metadata
    let meta_rec = WorldRow::get(supa, world_id).await?;

    let env_desc = meta_rec.environment
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
        world_epoch: meta_rec.world_epoch
            .as_ref()
            .and_then(|s| s.parse::<i128>().ok())
            .map(SimTime::from_ns),
    };

    let mut state = WorldState::new(meta, world_env);

    // 2. Load Objex templates ONCE
    /*let templates: HashMap<Uuid, Objex> =
        supa.select_objex_templates()
            .await?
            .into_iter()
            .map(|row| {
                let objex = Objex {
                    id: row.id,
                    geospec_id: row.geospec_id,
                    matcat: MatCatId::new(
                        row.matcat_category,
                        row.matcat_variant,
                        row.matcat_grade,
                    ),
                };
                (row.id, objex)
            })

            .collect();
*/
    // 3. Load entity rows
    let rows: Vec<EntityRow> =
        EntityRow::list_for_world(supa, world_id).await?;

    for row in rows {
        let id = EntityId(row.row_id.expect("EntityRow missing row_id"));

        // Register entity
        state.entities.insert(id);
        state.world_membership.insert(id, world_id);

        // Resolve template → attach components
        /*let template = templates
            .get(&row.objex_template_id)
            .expect("Missing Objex template");

        state.shapes.insert(id, ShapeRef {
            geospec_id: template.geospec_id,
        });

        state.materials.insert(id, MaterialRef {
            matcat: template.matcat,
        });
        */
        // Position (mandatory)
        let position: UvoxId =
            serde_json::from_value(row.position)
                .expect("Invalid UvoxId in DB");

        state.positions.insert(id, Position(position));

        // Orientation (optional)
        if !row.orientation.is_null() {
            let orientation =
                serde_json::from_value(row.orientation)
                    .expect("Invalid orientation in DB");

            state.orientations.insert(id, Orientation(orientation));
        }

        // Lifecycle
        state.lifecycles.insert(id, Lifecycle {
            spawned_at: row.spawned_at,
            despawned_at: row.despawned_at,
        });

        // Metadata
        state.metadata.insert(id, row.metadata);
    }

    Ok(state)
}
