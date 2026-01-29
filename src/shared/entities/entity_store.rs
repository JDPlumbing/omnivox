// shared/entity/entity_store.rs
use std::collections::HashMap;
use crate::core::EntityId;
use crate::core::entity::components::{time::Time, 
                              note::Note, 
                              world_membership::WorldMembership, 
                              position::Position,
                              spawned_at::SpawnedAt,
                              despawned_at::DespawnedAt,
                              active::Active,
                            };
use crate::core::entity::components::material::{Density, 
                                            Hardness, 
                                            Viscosity, 
                                            Conductivity
                                        };
use crate::core::entity::components::geometry::{Length, 
                                            Radius,     
                                            Thickness, 
                                            Width, 
                                            Height
                                        };
use crate::core::entity::components::spatial::{Velocity,
                                        VelocityENU,
                                        PositionENU,
                                        acceleration_enu::AccelerationENU,
                                        location::Location,
                                        


                                        };


#[derive(Debug, Default, Clone)]
pub struct EntityStore {
    pub lengths: HashMap<EntityId, Length>,
    pub radii: HashMap<EntityId, Radius>,
    pub thicknesses: HashMap<EntityId, Thickness>,
    pub widths: HashMap<EntityId, Width>,
    pub heights: HashMap<EntityId, Height>,
    pub densities: HashMap<EntityId, Density>,
    pub hardnesses: HashMap<EntityId, Hardness>,
    pub viscosities: HashMap<EntityId, Viscosity>,
    pub conductivities: HashMap<EntityId, Conductivity>,
    pub times: HashMap<EntityId, Time>,
    pub notes: HashMap<EntityId, Note>,
    pub world_memberships: HashMap<EntityId, WorldMembership>,
    pub positions: HashMap<EntityId, Position>,
    pub spawned_ats: HashMap<EntityId, SpawnedAt>,
    pub despawned_ats: HashMap<EntityId, DespawnedAt>,
    pub actives: HashMap<EntityId, Active>,
    pub velocities: HashMap<EntityId, Velocity>,
    pub position_enus: HashMap<EntityId, PositionENU>,
    pub velocity_enus: HashMap<EntityId, VelocityENU>,
    pub acceleration_enus: HashMap<EntityId, AccelerationENU>,
    pub locations: HashMap<EntityId, Location>,

}

impl EntityStore {
    pub fn new() -> Self {
        Self {
            lengths: HashMap::new(),
            radii: HashMap::new(),
            thicknesses: HashMap::new(),
            widths: HashMap::new(),
            heights: HashMap::new(),
            densities: HashMap::new(),
            hardnesses: HashMap::new(),
            viscosities: HashMap::new(),
            conductivities: HashMap::new(),
            times: HashMap::new(),
            notes: HashMap::new(),
            world_memberships: HashMap::new(),
            positions: HashMap::new(),
            spawned_ats: HashMap::new(),
            despawned_ats: HashMap::new(),
            actives: HashMap::new(),
            velocities: HashMap::new(),
            position_enus: HashMap::new(),
            velocity_enus: HashMap::new(),
            acceleration_enus: HashMap::new(),
            locations: HashMap::new(),
        }
    }

    pub fn add_time(&mut self, entity: EntityId, time: Time) {
        self.times.insert(entity, time);
    }

    pub fn add_note(&mut self, entity: EntityId, note: Note ) {
        self.notes.insert(entity, note);
    }

    pub fn add_world_membership(&mut self, entity: EntityId, membership: WorldMembership) {
        self.world_memberships.insert(entity, membership);
    }

    pub fn add_position(&mut self, entity: EntityId, position: Position) {
        self.positions.insert(entity, position);
    }
    pub fn add_spawned_at(&mut self, entity: EntityId, spawned_at: SpawnedAt) {
        self.spawned_ats.insert(entity, spawned_at);
    }
    pub fn add_despawned_at(&mut self, entity: EntityId, despawned_at: DespawnedAt) {
        self.despawned_ats.insert(entity, despawned_at);
    }
    pub fn add_active(&mut self, entity: EntityId) {
        self.actives.insert(entity, Active);
    }

    pub fn remove_active(&mut self, entity: &EntityId) {
        self.actives.remove(entity);
    }

    pub fn is_active(&self, entity: &EntityId) -> bool {
        self.actives.contains_key(entity)
    }
    pub fn add_density(&mut self, entity: EntityId, density: Density) {
        self.densities.insert(entity, density);
    }
    pub fn add_hardness(&mut self, entity: EntityId, hardness: Hardness) {
        self.hardnesses.insert(entity, hardness);
    }
    pub fn add_viscosity(&mut self, entity: EntityId, viscosity: Viscosity) {
        self.viscosities.insert(entity, viscosity);
    }
    pub fn add_conductivity(&mut self, entity: EntityId, conductivity: Conductivity) {
        self.conductivities.insert(entity, conductivity);
    }
    pub fn add_length(&mut self, entity: EntityId, length: Length) {
        self.lengths.insert(entity, length);
    }
    pub fn add_radius(&mut self, entity: EntityId, radius: Radius) {
        self.radii.insert(entity, radius);
    }
    pub fn add_thickness(&mut self, entity: EntityId, thickness: Thickness) {
        self.thicknesses.insert(entity, thickness);
    }
    pub fn add_width(&mut self, entity: EntityId, width: Width) {
        self.widths.insert(entity, width);
    }
    pub fn add_height(&mut self, entity: EntityId, height: Height) {
        self.heights.insert(entity, height);
    }
    pub fn add_velocity(&mut self, entity: EntityId, velocity: Velocity) {
        self.velocities.insert(entity, velocity);
    }
    pub fn add_position_enu(&mut self, entity: EntityId, position_enu: PositionENU) {
        self.position_enus.insert(entity, position_enu);
    }
    pub fn add_velocity_enu(&mut self, entity: EntityId, velocity_enu: VelocityENU) {
        self.velocity_enus.insert(entity, velocity_enu);
    }
    pub fn add_location(&mut self, entity: EntityId, location: Location) {
        self.locations.insert(entity, location);
    }
    pub fn add_acceleration_enu(&mut self, entity: EntityId, acceleration_enu: AccelerationENU) {
        self.acceleration_enus.insert(entity, acceleration_enu);
    }

}
