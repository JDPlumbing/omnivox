// shared/entity/entity_store.rs
use std::collections::HashMap;
use crate::core::entity::id::EntityId;
use crate::core::entity::components::entity_environment_sample::EntityEnvironmentSample;
use crate::core::entity::components::exposure::Exposure;
use crate::core::entity::components::{time::Time, 
                              note::Note, 
                              world_membership::WorldMembership, 
                              position::Position,
                              spawned_at::SpawnedAt,
                              despawned_at::DespawnedAt,
                              active::Active,
                              absorbed_energy::AbsorbedEnergy,
                              internal_energy::InternalEnergy,
                              mass::Mass,
                              temperature::Temperature,
                              weight::Weight,
                              geometry::Geometry,
                              geometry_parts::volume::Volume,
                              geometry_parts::surface_area::SurfaceArea,
                              exposure_area::ExposureArea,
                              material::Material,
                            };
use crate::core::entity::components::materials::{Density, 
                                            Hardness, 
                                            viscosity::Viscosity, 
                                            Absorptivity,
                                            thermal::specific_heat::SpecificHeat,
                                            emissivity::Emissivity
                                        };

use crate::core::entity::components::spatial::{Velocity,
                                        VelocityENU,
                                        PositionENU,
                                        AccelerationENU,
                                        Grounded,
                                        };

#[derive(Debug, Default, Clone)]
pub struct EntityStore {
    pub densities: HashMap<EntityId, Density>,
    pub hardnesses: HashMap<EntityId, Hardness>,
    pub viscosities: HashMap<EntityId, Viscosity>,
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
    pub groundeds: HashMap<EntityId, Grounded>,
    pub entity_environment_samples: HashMap<EntityId, EntityEnvironmentSample>,
    pub exposures: HashMap<EntityId, Exposure>,
    pub absorbed_energies: HashMap<EntityId, AbsorbedEnergy>,
    pub absorptivities: HashMap<EntityId, Absorptivity>,
    pub internal_energies: HashMap<EntityId, InternalEnergy>,
    pub specific_heats: HashMap<EntityId, SpecificHeat>,
    pub temperatures: HashMap<EntityId, Temperature>,
    pub masses: HashMap<EntityId, Mass>,
    pub weights: HashMap<EntityId, Weight>,
    pub emissivities: HashMap<EntityId, Emissivity>,
    pub geometries: HashMap<EntityId, Geometry>,
    pub volumes: HashMap<EntityId, Volume>,
    pub surface_areas: HashMap<EntityId, SurfaceArea>,
    pub exposure_areas: HashMap<EntityId, ExposureArea>,
    pub materials: HashMap<EntityId, Material>,
}

impl EntityStore {
    pub fn new() -> Self {
        Self {
            densities: HashMap::new(),
            hardnesses: HashMap::new(),
            viscosities: HashMap::new(),
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
            groundeds: HashMap::new(),
            entity_environment_samples: HashMap::new(), 
            exposures: HashMap::new(),
            absorbed_energies: HashMap::new(),
            absorptivities: HashMap::new(),
            internal_energies: HashMap::new(),
            specific_heats: HashMap::new(),
            temperatures: HashMap::new(),
            emissivities: HashMap::new(),
            masses: HashMap::new(),
            weights: HashMap::new(),
            geometries: HashMap::new(),
            volumes: HashMap::new(),
            surface_areas: HashMap::new(),
            exposure_areas: HashMap::new(),
            materials: HashMap::new(),
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
    pub fn add_absorbed_energy(&mut self, entity: EntityId, absorbed_energy: AbsorbedEnergy) {
        self.absorbed_energies.insert(entity, absorbed_energy);
    }
    pub fn add_absorptivity(&mut self, entity: EntityId, absorptivity: Absorptivity) {
        self.absorptivities.insert(entity, absorptivity);
    }
    pub fn add_exposure_area(&mut self, entity: EntityId, exposure_area: ExposureArea) {
        self.exposure_areas.insert(entity, exposure_area);
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

    pub fn add_velocity(&mut self, entity: EntityId, velocity: Velocity) {
        self.velocities.insert(entity, velocity);
    }
    pub fn add_position_enu(&mut self, entity: EntityId, position_enu: PositionENU) {
        self.position_enus.insert(entity, position_enu);
    }
    pub fn add_velocity_enu(&mut self, entity: EntityId, velocity_enu: VelocityENU) {
        self.velocity_enus.insert(entity, velocity_enu);
    }

    pub fn add_acceleration_enu(&mut self, entity: EntityId, acceleration_enu: AccelerationENU) {
        self.acceleration_enus.insert(entity, acceleration_enu);
    }

    pub fn add_grounded(&mut self, entity: EntityId, grounded: Grounded) {
        self.groundeds.insert(entity, grounded);
    }
    pub fn add_entity_environment_sample(&mut self, entity: EntityId, sample: EntityEnvironmentSample) {
        self.entity_environment_samples.insert(entity, sample);
    }
    pub fn add_exposure(&mut self, entity: EntityId, exposure: Exposure) {
        self.exposures.insert(entity, exposure);
    }
    pub fn add_internal_energy(&mut self, entity: EntityId, internal_energy: InternalEnergy) {
        self.internal_energies.insert(entity, internal_energy);
    }
    pub fn add_specific_heat(&mut self, entity: EntityId, specific_heat: SpecificHeat) {
        self.specific_heats.insert(entity, specific_heat);
    }
    pub fn add_temperature(&mut self, entity: EntityId, temperature: Temperature) {
        self.temperatures.insert(entity, temperature);
    }
    pub fn add_emissivity(&mut self, entity: EntityId, emissivity: Emissivity) {
        self.emissivities.insert(entity, emissivity);
    }   

    pub fn add_mass(&mut self, entity: EntityId, mass: Mass) {
        self.masses.insert(entity, mass);
    }
    pub fn add_weight(&mut self, entity: EntityId, weight: Weight) {
        self.weights.insert(entity, weight);
    }
    pub fn add_geometry(&mut self, entity: EntityId, geometry: Geometry) {
        self.geometries.insert(entity, geometry);
    }
    pub fn add_volume(&mut self, entity: EntityId, volume: Volume) {
        self.volumes.insert(entity, volume);
    }
    pub fn add_surface_area(&mut self, entity: EntityId, surface_area: SurfaceArea) {
        self.surface_areas.insert(entity, surface_area);
    }
    pub fn add_material(&mut self, entity: EntityId, material: Material) {
        self.materials.insert(entity, material);
    }
}
