use crate::{
    objex::core::{Objex, Object},
    objex::systems::thermal::{derive_thermal, ThermalProps},
    sim::{systems::System, world::WorldState},
    

    matcat::materials::{props_for, default_props},
    chronovox::ChronoEvent,
    objex::Shape,
};
use uuid::Uuid;
use crate::geospec::traits::Volume;
pub struct ThermalSystem;

impl System for ThermalSystem {
    fn name(&self) -> &'static str {
        "thermal"
    }

    fn tick(&mut self, world: &mut WorldState) -> Vec<ChronoEvent> {
        let mut events = Vec::new();

        for (id, objex) in &world.objects {
            let mat = if let Some(mat_id) = &objex.material.matcat_id {
                props_for(mat_id)
            } else {
                default_props()
            };

            let object = Object {
                shape: objex.shape.clone(),
                material: mat,
            };

            let thickness = object.shape.volume().cbrt() * 0.1;
            let props = derive_thermal(&object);
            
            world.thermal_components.insert(Uuid::parse_str(id).unwrap_or_default(), props);

        }

        events
    }
}
