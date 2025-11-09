use omnivox::{
    sim::{
        systems::{
            collision::CollisionSystem, fracture::FractureSystem, movement::MovementSystem,
            degradation::DegradationSystem, corrosion::CorrosionSystem, thermal::ThermalSystem,
            electrical::ElectricalSystem, optical::OpticalSystem, composite::CompositeSystem,
            System,
        },
        world::WorldState,
        components::Velocity,
        clock::SimClock,
    },
    objex::core::{Objex, MaterialLink, MaterialName, Shape},
    matcat::materials::MatCatId,
};
use uuid::Uuid;
use chrono::{Utc, Duration};
use std::fs::File;
use std::io::Write;
use serde_json::json;

fn main() {
    const EARTH_RADIUS: i64 = 6_371_000_000_000;
    let mut world = WorldState::default();
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
    ];

    // 🕒 SimClock: 10 years, 1-month steps
    let now = Utc::now();
    let start = now - Duration::days(365 * 10);
    let step = Duration::days(30);
    let mut clock = SimClock::new(start, now, step);

    // ⚙️ Setup object
    let obj_id = Uuid::new_v4().to_string();
    let material = MaterialLink::new(MaterialName::Copper);
    let shape = Shape::Sphere(omnivox::geospec::shapes::Sphere { radius: 1.0 });
    let mut obj = Objex::new(0, None, "test_ball", shape, material);
    obj.uvoxid.r_um = EARTH_RADIUS - 10;
    obj.material.matcat_id = Some(MatCatId::metal_cu());
    world.objects.insert(obj_id.clone(), obj);
    world.velocity_components.insert(
        Uuid::parse_str(&obj_id).unwrap(),
        Velocity { dr: -100.0, dlat: 0.0, dlon: 0.0 },
    );

    // 📂 Open output file
    std::fs::create_dir_all("dev").expect("failed to create dev directory");
    let mut file = File::create("dev/output.json").expect("failed to create output file");

    let mut all_events = Vec::new();

    // 🧮 Main simulation loop
    while clock.current < clock.end {
        let mut tick_events = vec![];
        for system in systems.iter_mut() {
            let events = system.tick(&mut world);
            if !events.is_empty() {
                tick_events.extend(events);
            }
        }

        if !tick_events.is_empty() {
            all_events.push(json!({
                "tick_time": clock.current.to_string(),
                "events": tick_events
            }));
        }

        clock.advance();
    }

    // 💾 Write all events to file
    let json_output = serde_json::to_string_pretty(&all_events).unwrap();
    file.write_all(json_output.as_bytes()).unwrap();

    println!("✅ Simulation complete. Output written to dev/output.json");
}
