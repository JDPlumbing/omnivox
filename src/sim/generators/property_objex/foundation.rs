use crate::objex::core::{Objex, MaterialLink, MaterialName, Shape};
use crate::supabasic::properties::PropertyRecord;

/// Generate foundation-related Objex entities for a property.
///
/// Uses statistical estimates to infer geometry based on total square footage.
/// Returns:
/// - 1 slab objex (main floor pad)
/// - 1 footing objex (perimeter beam)
pub fn generate_foundation_objex(property: &PropertyRecord) -> Vec<Objex> {
    let sqft = property.square_feet.unwrap_or(1600) as f64;
    let aspect_ratio = 1.4; // average single-family aspect ratio
    let width = (sqft / aspect_ratio).sqrt();
    let length = width * aspect_ratio;
    let perimeter = 2.0 * (length + width);

    let frame_id = property.frame_id.unwrap_or(0);
    let property_id = property.property_id.unwrap_or_default();

    let mut objexes = Vec::new();

    // 🧱 Foundation slab
    let slab = Objex {
        frame_id,
        property_id: Some(property_id),
        entity_id: uuid::Uuid::new_v4(),
        name: format!("Foundation Slab ({:.0} sqft)", sqft),
        shape: Shape::Box(crate::geospec::shapes::BoxShape {
            length,
            width,
            height: 0.5, // 6" slab
        }),
        material: MaterialLink::new(MaterialName::Concrete),
    };
    objexes.push(slab);

    // 🧱 Perimeter footing
    let footing = Objex {
        frame_id,
        property_id: Some(property_id),
        entity_id: uuid::Uuid::new_v4(),
        name: format!("Perimeter Footing ({:.0} ft perimeter)", perimeter),
        shape: Shape::Box(crate::geospec::shapes::BoxShape {
            length: perimeter / 4.0, // simplified linear section representation
            width: 2.0,              // 2 ft wide
            height: 2.0,             // 2 ft deep
        }),
        material: MaterialLink::new(MaterialName::Concrete),
    };
    objexes.push(footing);

    objexes
}
