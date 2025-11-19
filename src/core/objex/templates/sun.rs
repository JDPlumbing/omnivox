use crate::core::{
    objex::core::types::Objex,
    objex::core::material::{MaterialName, MaterialLink},
    
    uvoxid::UvoxId,
};
use crate::sim::components::{OrbitalMotion, SunEmitter},
pub struct Sun;

impl Sun {
    pub fn create() -> (Objex, OrbitalMotion, SunEmitter) {
        let mat = MaterialLink::new(MaterialName::Plasma);
        let radius_m = 6.9634e8;

        let mut obj = Objex::new_sphere(0, None, mat, radius_m)
            .with_metadata("type", "star")
            .with_metadata("emissive", "true");

        obj.name = "Sun".to_string();
        obj.uvoxid = UvoxId::new(
            0,
            149_597_870_700_000_000, // correct AU in μm

            0,
            0,
        );

        let orbital = OrbitalMotion::new_for_sun();
        let emitter = SunEmitter::new_for_sun();

        (obj, orbital, emitter)
    }


}
