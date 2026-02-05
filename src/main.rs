use omnivox::core::cosmic::state::CosmicState;
use omnivox::core::worlds::state::WorldState;
use omnivox::core::environment::state::EnvironmentState;
use omnivox::core::simulation::sim_engine::SimulationEngine;
use omnivox::core::render::view::ViewFrame;
use omnivox::core::tdt::sim_time::SimTime;
use omnivox::shared::entities::entity_store::EntityStore;

use omnivox::core::physics::units::{length::Meters, angle::Radians, angle::Degrees};
use omnivox::core::worlds::id::WorldId;
use omnivox::core::spatial::surface_coords::SurfaceCoords;
use omnivox::core::tdt::TimeContext;

use omnivox::core::entity::constructors::geometry::round_tube::build_round_tube;
use omnivox::core::entity::constructors::materials::copper::copper;
use omnivox::core::entity::constructors::spawn_physical_object::{
    spawn_physical_object, SpawnPhysicalObjectArgs,
};
use omnivox::core::commands::context::CommandContext;
use omnivox::core::commands::create_round_tube::CreateRoundTube;
use omnivox::core::render::camera::{self, Camera};
use omnivox::core::math::vec3::Vec3;
use omnivox::core::render::ascii;
use omnivox::core::cosmic::systems::frame_system::CosmicFrameSystem;
use omnivox::core::cosmic::id::CosmicBodyId;
use omnivox::core::cosmic::debug::log_subsolar_point;

fn main() {
    // -----------------------------
    // Build world state
    // -----------------------------
    let cosmic = CosmicState::demo_solar_system();
    let world_state = WorldState::demo_worlds();
    let environment = EnvironmentState::demo_earth();

    // -----------------------------
    // Build engine FIRST
    // -----------------------------
    let mut engine = SimulationEngine::new_with_full_state(
        SimTime::from_seconds_f64(0.0),
        60_000_000_000, // 1 minute per tick
        cosmic,
        world_state,
        environment,
        EntityStore::default(),
    );


    // -----------------------------
    // Build geometry & material
    // -----------------------------
    let geometry = build_round_tube(
        Meters(0.05),
        Meters(0.045),
        Meters(1.0),
    );

    let material = copper();

    // -----------------------------
    // Spawn physical entity
    // -----------------------------


    let mut cmd = CommandContext::new(&mut engine);

    let entity = CreateRoundTube {
        world_id: WorldId(1),
        location: SurfaceCoords {
            latitude: Degrees(26.1224_f64.to_radians()),
            longitude: Degrees(-80.1373_f64.to_radians()),
            elevation: Meters(0.0),
        },
        outer_radius: Meters(0.05),
        inner_radius: Meters(0.045),
        length: Meters(1.0),
        grounded: false,
    }.apply(&mut cmd);


    println!("Spawned entity {:?}", entity);

    // -----------------------------
    // Camera
    // -----------------------------
    let camera = Camera {
        center: Vec3::ZERO,
        scale: 1.0 / 2.0e11,
    };

    // -----------------------------
    // Main loop
    // -----------------------------
    loop {
        print!("\x1B[2J\x1B[H");

        engine.tick();

        // -----------------------------
        // DEBUG: subsolar point
        // -----------------------------
        {
            let frame = CosmicFrameSystem {
                state: &engine.state.cosmic,
            };

            let sun   = CosmicBodyId(1);
            let earth = CosmicBodyId(2);

            log_subsolar_point(
                &frame,
                earth,
                sun,
                engine.time,
            );
        }


        let primitives = engine.render_view(&ViewFrame::Cosmic);
        let projected: Vec<_> = primitives
            .iter()
            .filter_map(|p| camera::project(p, &camera))
            .collect();

        //ascii::render_ascii(&projected);

        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}
