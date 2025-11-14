use omnivox::{
    sim::{
        systems::{
            collision::CollisionSystem,
            fracture::FractureSystem,
            movement::MovementSystem,
            degradation::DegradationSystem,
            corrosion::CorrosionSystem,
            thermal::ThermalSystem,
            electrical::ElectricalSystem,
            optical::OpticalSystem,
            composite::CompositeSystem,
            SolarMotionSystem,
            SolarRaycastSystem,
            SolarExposureSystem,
            UVDegradationSystem,
            System,
        },
        world::WorldState,
        components::Velocity,
        clock::SimClock,
    },
    objex::core::{Objex, MaterialLink, MaterialName, Shape},
    matcat::materials::MatCatId,
    geospec::shapes::Sphere,
};
use uuid::Uuid;
use chrono::{Utc, Duration};
use std::{fs::File, io::Write};
use serde_json::json;
use omnivox::objex::templates::Sun;

fn main() {
    const EARTH_RADIUS: i64 = 6_371_000_000_000;

    let mut world = WorldState::default();

    // 🌞 Add the Sun
    let (sun_obj, orbit, emitter) = Sun::create();
    let sun_uuid = sun_obj.entity_id;

    world.objects.insert(sun_uuid.to_string(), sun_obj);
    world.orbital_components.insert(sun_uuid, orbit);
    world.sun_emitter_components.insert(sun_uuid, emitter);

    // 🧠 Add systems
    let mut systems: Vec<Box<dyn System>> = vec![
        Box::new(MovementSystem),
        Box::new(CollisionSystem),
        Box::new(FractureSystem),
        Box::new(DegradationSystem),
        Box::new(CorrosionSystem),
        Box::new(ThermalSystem),
        Box::new(ElectricalSystem),
        Box::new(OpticalSystem),
        Box::new(CompositeSystem),

        // 🌞 New solar systems
        Box::new(SolarMotionSystem),
        Box::new(SolarRaycastSystem),
        Box::new(SolarExposureSystem),
        Box::new(UVDegradationSystem),
    ];

    // 🕒 Clock
    let now = Utc::now();
    let start = now - Duration::days(1);
    let step = Duration::days(1);

    let mut clock = SimClock::from_wall_dates(start, now, step);

    // 🎯 Add test object
    let obj_uuid = Uuid::new_v4();
    let obj_id = obj_uuid.to_string();

    let material = MaterialLink::new(MaterialName::Copper);
    let shape = Shape::Sphere(Sphere { radius: 1.0 });

    let mut obj = Objex::new(0, None, "test_ball", shape, material);
    obj.uvoxid.r_um = EARTH_RADIUS - 10;
    obj.material.matcat_id = Some(MatCatId::metal_cu());

    world.objects.insert(obj_id.clone(), obj);

    world.velocity_components.insert(
        obj_uuid,
        Velocity { dr: -100.0, dlat: 0.0, dlon: 0.0 },
    );

    // 📁 Output file
    std::fs::create_dir_all("dev").expect("failed to create dev directory");
    let mut file = File::create("dev/output.json").expect("failed to create output file");

    let mut all_events = Vec::new();

    // 🔁 Sim loop
    while clock.current_ns < clock.end_ns {
        // Make the world aware of the current clock state
        world.clock = Some(clock.clone());

        let mut tick_events = vec![];

        // Tick all systems
        for system in systems.iter_mut() {
            let events = system.tick(&mut world);
            if !events.is_empty() {
                tick_events.extend(events);
            }
        }

        if !tick_events.is_empty() {
            all_events.push(json!({
                "tick_time_ns": clock.current_ns,
                "events": tick_events
            }));
        }

        // Advance to next time
        clock.advance();
    }

    // 💾 Save JSON log
    let json_output = serde_json::to_string_pretty(&all_events).unwrap();
    file.write_all(json_output.as_bytes()).unwrap();

    println!("✅ Simulation complete. Output written to dev/output.json");
}
