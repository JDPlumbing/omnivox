use crate::supabasic::properties::PropertyRecord;
use crate::objex::{Objex, Shape, MaterialLink};
use crate::objex::core::{MaterialName, MaterialKind};
use crate::geospec::shapes::BoxShape;
use uuid::Uuid;

pub fn generate_roof(property: &PropertyRecord) -> Vec<Objex> {
    let mut objs = Vec::new();

    let sqft = property.square_feet.unwrap_or(1200) as f64;
    let side_length = sqft.sqrt();
    let overhang = 1.0; // ft
    let pitch_ratio = 6.0 / 12.0; // 6:12 pitch

    let roof_length = side_length + 2.0 * overhang;
    let roof_width  = side_length + 2.0 * overhang;
    let rise = (roof_width / 2.0) * pitch_ratio;
    let roof_height = rise; // approximate

    let roof_material = MaterialLink {
        id: Uuid::new_v4(),
        name: MaterialName::Custom("AsphaltShingle".into()),
        kind: MaterialKind::Composite,
    };

    let shape = Shape::Box(BoxShape {
        length: roof_length,
        width: roof_width,
        height: roof_height,
    });

    objs.push(Objex {
        frame_id: property.frame_id.unwrap_or(0),
        entity_id: Uuid::new_v4(),
        property_id: property.property_id,
        name: format!("Roof ({} sqft)", sqft.round()),
        shape,
        material: roof_material,
    });

    objs
}

