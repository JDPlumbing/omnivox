use uuid::Uuid;
use crate::objex::core::{Objex, Shape, MaterialLink, MaterialName};

use crate::supabasic::properties::{PropertyRecord, PlumbingType};
use crate::geospec::shapes::{Line, Cylinder, BoxShape};

pub fn generate_plumbing_objex(property: &PropertyRecord) -> Vec<Objex> {
    let mut objs = Vec::new();

    // Pick material based on plumbing type
    let mat_name = match property.plumbing {
        Some(PlumbingType::Copper) => MaterialName::Copper,
        Some(PlumbingType::PEX) => MaterialName::Plastic,
        Some(PlumbingType::CPVC) => MaterialName::Plastic,
        Some(PlumbingType::Galvanized) => MaterialName::Steel,
        Some(PlumbingType::CastIron) => MaterialName::Steel,
        _ => MaterialName::Custom("Mixed".into()),
    };
    let material = MaterialLink::new(mat_name);

    let frame_id = property.frame_id.unwrap_or(0);
    let prop_id = property.property_id;
    let sqft = property.square_feet.unwrap_or(1200);
    let bathrooms = property.bathrooms.unwrap_or(1);
    let fixture_groups = bathrooms + if sqft > 1200 { 1 } else { 0 };

    // 🚰 Main water service
    objs.push(Objex::new(
        frame_id,
        prop_id,
        "MainWaterService",
        Shape::Line(Line { length: 25.0 }),
        MaterialLink::new(MaterialName::Copper),
    ));

    // 🔥 Water heater
    objs.push(Objex::new(
        frame_id,
        prop_id,
        "WaterHeater",
        Shape::Cylinder(Cylinder {
            radius: 1.0,
            height: if sqft > 2500 { 6.0 } else { 5.0 },
        }),
        MaterialLink::new(MaterialName::Steel),
    ));

    // 💧 Supply piping
    objs.push(Objex::new(
        frame_id,
        prop_id,
        "SupplyPiping",
        Shape::Line(Line {
            length: (sqft as f64 / 10.0),
        }),
        material.clone(),
    ));

    // 🚽 Drain-waste-vent system
    objs.push(Objex::new(
        frame_id,
        prop_id,
        "DrainWasteVent",
        Shape::Line(Line { length: (sqft as f64 / 12.0) }),
        MaterialLink::new(MaterialName::Plastic),
    ));

    // 🚿 Fixture clusters
    for i in 0..fixture_groups {
        objs.push(Objex::new(
            frame_id,
            prop_id,
            format!("FixtureCluster_{}", i + 1),
            Shape::Box(BoxShape {
                length: 3.0,
                width: 3.0,
                height: 8.0,
            }),
            material.clone(),
        ));
    }

    objs
}
