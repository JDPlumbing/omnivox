use crate::objex::core::types::MaterialLink;


pub fn default_material_for_r_um(r_um: i64) -> MaterialLink {
    use crate::objex::core::types::{MaterialLink, MaterialName};

    const EARTH_RADIUS: i64 = 6_371_000_000_000;
    if r_um < EARTH_RADIUS - 30_000_000_000 {
        MaterialLink::new(MaterialName::Steel)
    } else if r_um < EARTH_RADIUS {
        MaterialLink::new(MaterialName::Concrete)
    } else if r_um < EARTH_RADIUS + 10_000_000_000 {
        MaterialLink::new(MaterialName::Air)
    } else {
        MaterialLink::vacuum()
    }
}
