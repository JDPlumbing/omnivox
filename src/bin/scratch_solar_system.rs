use omnivox::core::tdt::{SimDuration, SimTime};
use omnivox::core::uvoxid::UvoxId;

use omnivox::sim::world::state::World;
use omnivox::sim::world::WorldState;

use chrono::{TimeZone, Duration as ChronoDuration};
use omnivox::sim::systems::{
    System,
    SolarRadiationSystem,
    SolarExposureSystem,
    SunDamageSystem,
};

use omnivox::sim::components::{
    SolarRadiation,
    SolarExposure,
    SunDamage,
};

use omnivox::core::physox::astronomy::solar::solar_uvox;
use omnivox::core::objex::core::{Objex, MaterialLink};
use omnivox::core::objex::geospec::shapes::Shape;

use omnivox::sim::time::clock::SimClock;
use omnivox::core::id::entity_id::EntityId;
use omnivox::core::id::world_id::WorldId;
use omnivox::core::id::simulation_id::SimulationId;

use uuid::Uuid;
use serde_json::Value;

fn main() {
    println!("========== SOLAR SYSTEM SCRATCH TEST ==========\n");

    // ---------------------------------------------------------
    // World
    // ---------------------------------------------------------
    let world_meta = World {
        id: WorldId(1),
        name: Some("TestWorld".into()),
        description: Some("Solar scratch test".into()),
        world_epoch: Some(SimTime::from_ns(0)),
    };

    let mut world = WorldState::new(world_meta);

    // ---------------------------------------------------------
    // Initial time — Miami daytime
    // ---------------------------------------------------------
    let sim_t = SimTime::from_datetime(
        chrono::Utc.with_ymd_and_hms(2025, 3, 10, 17, 0, 0).unwrap()
    );

    // Set simulation time
    world.sim_time = sim_t;

    // Set delta step (1 hour)
    let dt = SimDuration::from_seconds(3600);
    world.sim_delta = dt;

    // ---------------------------------------------------------
    // Create CLOCK (this is what was missing!)
    // ---------------------------------------------------------
    world.clock = Some(SimClock {
        start: sim_t,
        current: sim_t,
        end: sim_t.add_seconds(3600 * 36),   // simulate 12 hours
        step: dt,
    });

    // ---------------------------------------------------------
    // SUN entity
    // ---------------------------------------------------------
    let sun_id = EntityId::new(0, 0);
    let sun_uv = solar_uvox(sim_t);

    let sun_entity = omnivox::sim::entities::SimEntity {
        id: sun_id,
        world_id: WorldId(1),
        spawned_at: sim_t,
        template: Objex::sphere(MaterialLink::stellar_plasma(), 1000.0),
        orientation: Default::default(),
        despawned_at: None,
        metadata: Value::Null,
        position: sun_uv,
    };

    world.entities.insert(sun_id, sun_entity);
    world.components
         .solar_radiation
         .insert(sun_id, SolarRadiation::sun_default());

    // ---------------------------------------------------------
    // Ground entity — Miami
    // ---------------------------------------------------------
    let ground_id = EntityId::new(1, 0);
    let lat = 25.7617;
    let lon = -80.1918;
    let r_um = (6_371_000.0 * 1e6) as i64;

    let ground_pos = UvoxId::new(
        r_um,
        (lat * 1e11) as i64,
        (lon * 1e11) as i64,
    );

    let ground = omnivox::sim::entities::SimEntity {
        id: ground_id,
        world_id: WorldId(1),
        spawned_at: sim_t,
        template: Objex::box_shape(MaterialLink::vacuum(), 1.0, 1.0, 1.0),
        orientation: Default::default(),
        despawned_at: None,
        metadata: Value::Null,
        position: ground_pos,
    };

    world.entities.insert(ground_id, ground);

    // ---------------------------------------------------------
    // Systems
    // ---------------------------------------------------------
    let mut rad_sys = SolarRadiationSystem::default();
    let mut exp_sys = SolarExposureSystem::default();
    let mut dmg_sys = SunDamageSystem::default();

    // ---------------------------------------------------------
    // Simulation loop — 12 hours
    // ---------------------------------------------------------
    for _ in 0..12 {
        // Advance clock
        if let Some(clock) = &mut world.clock {
            clock.advance();
            world.sim_time = clock.current;
        }

        rad_sys.tick(&mut world);
        exp_sys.tick(&mut world);
        dmg_sys.tick(&mut world);
    }

    // ---------------------------------------------------------
    // Output results
    // ---------------------------------------------------------
    println!("SolarExposure = {:?}", world.components.solar_exposure.get(&ground_id));
    println!("SunDamage     = {:?}", world.components.sun_damage.get(&ground_id));

    println!("\n========== END ==========");
}
