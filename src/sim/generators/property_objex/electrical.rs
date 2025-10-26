use uuid::Uuid;
use crate::objex::core::{Objex, Shape, MaterialLink, MaterialName};

use crate::supabasic::properties::{PropertyRecord, ElectricalType};
use crate::geospec::shapes::{Line, BoxShape};

pub fn generate_electrical_objex(property: &PropertyRecord) -> Vec<Objex> {
    let mut objs = Vec::new();

    let mat_name = match property.electrical {
        Some(ElectricalType::Copper) => MaterialName::Copper,
        Some(ElectricalType::Aluminum) => MaterialName::Aluminum,
        Some(ElectricalType::Mixed) => MaterialName::Custom("Mixed".into()),
        _ => MaterialName::Custom("Unknown".into()),
    };
    let material = MaterialLink::new(mat_name);

    let frame_id = property.frame_id.unwrap_or(0);
    let prop_id = property.property_id;
    let sqft = property.square_feet.unwrap_or(1200);
    let bathrooms = property.bathrooms.unwrap_or(1);

    let circuits = ((sqft as f64 / 100.0).ceil() as i64) + bathrooms * 2;

    // ⚡ Service panel
    objs.push(Objex::new(
        frame_id,
        prop_id,
        "ServicePanel",
        Shape::Box(BoxShape {
            length: 2.0,
            width: 0.5,
            height: 3.0,
        }),
        MaterialLink::new(MaterialName::Steel),
    ));

    // 🔌 Branch circuits
    objs.push(Objex::new(
        frame_id,
        prop_id,
        "BranchCircuits",
        Shape::Line(Line { length: sqft as f64 / 8.0 }),
        material.clone(),
    ));

    // 💡 Lighting groups
    let lighting_groups = ((sqft as f64) / 400.0).ceil() as i64;
    for i in 0..lighting_groups {
        objs.push(Objex::new(
            frame_id,
            prop_id,
            format!("LightingGroup_{}", i + 1),
            Shape::Box(BoxShape {
                length: 5.0,
                width: 5.0,
                height: 3.0,
            }),
            material.clone(),
        ));
    }

    // ⚙️ Appliances
    objs.push(Objex::new(
        frame_id,
        prop_id,
        "HVAC_Circuit",
        Shape::Line(Line { length: 20.0 }),
        material.clone(),
    ));

    objs.push(Objex::new(
        frame_id,
        prop_id,
        "RangeCircuit",
        Shape::Line(Line { length: 15.0 }),
        material.clone(),
    ));

    objs
}
