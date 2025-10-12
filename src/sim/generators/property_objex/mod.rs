pub mod foundation;
pub mod walls;
pub mod roof;

use crate::supabasic::properties::PropertyRecord;
use crate::objex::Objex;

/// Generate all Objex entities associated with a given property.
/// Currently generates: foundation
pub fn generate_property_objexes(property: &PropertyRecord) -> Vec<Objex> {
    let mut all = Vec::new();

    // Foundation (slab, footing, etc.)
    all.extend(foundation::generate_foundation_objex(property));

    // TODO: walls, roof, plumbing, electrical, etc.
    all.extend(walls::generate_walls(property));
    all.extend(roof::generate_roof(property));

    all
}
