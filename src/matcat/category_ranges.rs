use rand::Rng;
use crate::matcat::materials::MatProps;

pub struct PropRange {
    // Mechanical
    pub density: (f32, f32),
    pub elastic_modulus: (f32, f32),
    pub tensile_strength: (f32, f32),
    pub compressive_strength: (f32, f32),
    pub hardness: (f32, f32),
    pub fracture_toughness: (f32, f32),
    pub fatigue_resistance: (f32, f32),

    // Thermal
    pub thermal_conductivity: (f32, f32),
    pub thermal_expansion: (f32, f32),
    pub melting_point: (f32, f32),
    pub specific_heat: (f32, f32),

    // Chemical
    pub corrosion_resistance: (f32, f32),
    pub solubility: (f32, f32),
    pub permeability: (f32, f32),
    pub flammability: (f32, f32),

    // Electrical / Magnetic
    pub electrical_conductivity: (f32, f32),
    pub magnetic_permeability: (f32, f32),

    // Optical
    pub refractive_index: (f32, f32),
    pub transparency: (f32, f32),
    pub reflectivity: (f32, f32),
    pub absorption: (f32, f32),

    // UV Resistance
    pub uv_resistance: (f32, f32),
    
}


/// 20 baseline categories mapped to ID bytes
pub enum MatCategory {
    Metal = 0x01,
    SoftMetal = 0x02,
    Alloy = 0x03,
    Plastic = 0x04,
    Rubber = 0x05,
    Foam = 0x06,
    Wood = 0x07,
    Stone = 0x08,
    Concrete = 0x09,
    Ceramic = 0x0A,
    Glass = 0x0B,
    Liquid = 0x0C,
    Gas = 0x0D,
    Composite = 0x0E,
    Fabric = 0x0F,
    Paper = 0x10,
    Organic = 0x11,
    Ice = 0x12,
    Soil = 0x13,
    Nanomaterial = 0x14,
}


/// Lookup table for category ranges
pub fn get_category_ranges(cat: u8) -> Option<PropRange> {
    match cat {
        0x01 => Some(PropRange { // Metal
            density: (2700.0, 8900.0),
            elastic_modulus: (7e10, 2.1e11),
            tensile_strength: (200.0, 2000.0),
            compressive_strength: (300.0, 3000.0),
            hardness: (3.0, 9.0),
            fracture_toughness: (10.0, 150.0),
            fatigue_resistance: (0.6, 1.0),
            thermal_conductivity: (15.0, 400.0),
            thermal_expansion: (1e-6, 2.5e-5),
            melting_point: (600.0, 3400.0),
            specific_heat: (300.0, 900.0),
            corrosion_resistance: (0.1, 0.9),
            solubility: (0.0, 0.01),
            permeability: (0.0, 0.1),
            flammability: (0.0, 0.1),
            electrical_conductivity: (1e6, 1e7),
            magnetic_permeability: (1.0, 1000.0),
            refractive_index: (1.0, 2.5),
            transparency: (0.0, 1.0),
            reflectivity: (0.0, 1.0),
            absorption: (0.0, 1.0),
            uv_resistance: (0.0, 1.0),

        }),
        0x02 => Some(PropRange { // Alloy
            density: (5000.0, 15000.0),
            elastic_modulus: (1e11, 2.5e11),
            tensile_strength: (300.0, 2200.0),
            compressive_strength: (500.0, 3500.0),
            hardness: (4.0, 9.0),
            fracture_toughness: (15.0, 200.0),
            fatigue_resistance: (0.7, 1.0),
            thermal_conductivity: (10.0, 250.0),
            thermal_expansion: (5e-6, 2.0e-5),
            melting_point: (800.0, 3200.0),
            specific_heat: (300.0, 900.0),
            corrosion_resistance: (0.2, 0.95),
            solubility: (0.0, 0.02),
            permeability: (0.0, 0.2),
            flammability: (0.0, 0.1),
            electrical_conductivity: (1e5, 8e6),
            magnetic_permeability: (1.0, 2000.0),
            refractive_index: (1.0, 2.5),
            transparency: (0.0, 1.0),
            reflectivity: (0.0, 1.0),
            absorption: (0.0, 1.0),
            uv_resistance: (0.0, 1.0),

        }),
        0x03 => Some(PropRange { // Ceramic
            density: (2000.0, 6500.0),
            elastic_modulus: (1e10, 4e11),
            tensile_strength: (50.0, 500.0),
            compressive_strength: (500.0, 5000.0),
            hardness: (6.0, 9.0),
            fracture_toughness: (2.0, 15.0),
            fatigue_resistance: (0.1, 0.4),
            thermal_conductivity: (1.0, 60.0),
            thermal_expansion: (2e-6, 1.0e-5),
            melting_point: (1000.0, 3000.0),
            specific_heat: (500.0, 1500.0),
            corrosion_resistance: (0.7, 1.0),
            solubility: (0.0, 0.05),
            permeability: (0.0, 0.1),
            flammability: (0.0, 0.1),
            electrical_conductivity: (0.0, 10.0),
            magnetic_permeability: (0.0, 2.0),
            refractive_index: (1.0, 2.5),
            transparency: (0.0, 1.0),
            reflectivity: (0.0, 1.0),
            absorption: (0.0, 1.0),
            uv_resistance: (0.0, 1.0),

        }),
        0x04 => Some(PropRange { // Plastic
            density: (800.0, 2200.0),
            elastic_modulus: (1e8, 1e10),
            tensile_strength: (20.0, 100.0),
            compressive_strength: (40.0, 300.0),
            hardness: (1.0, 3.0),
            fracture_toughness: (0.5, 5.0),
            fatigue_resistance: (0.4, 0.8),
            thermal_conductivity: (0.1, 0.5),
            thermal_expansion: (5e-5, 2e-4),
            melting_point: (100.0, 400.0),
            specific_heat: (100.0, 2000.0),
            corrosion_resistance: (0.7, 1.0),
            solubility: (0.0, 0.5),
            permeability: (0.1, 0.9),
            flammability: (0.5, 1.0),
            electrical_conductivity: (0.0, 1.0),
            magnetic_permeability: (0.0, 1.0),
            refractive_index: (1.0, 2.5),
            transparency: (0.0, 1.0),
            reflectivity: (0.0, 1.0),
            absorption: (0.0, 1.0),
            uv_resistance: (0.0, 1.0),

        }),
        0x05 => Some(PropRange { // Foam
            density: (10.0, 500.0),
            elastic_modulus: (1e5, 1e8),
            tensile_strength: (1.0, 20.0),
            compressive_strength: (2.0, 50.0),
            hardness: (0.1, 1.0),
            fracture_toughness: (0.1, 2.0),
            fatigue_resistance: (0.2, 0.6),
            thermal_conductivity: (0.01, 0.2),
            thermal_expansion: (2e-5, 2e-4),
            melting_point: (50.0, 300.0),
            specific_heat: (500.0, 3000.0),
            corrosion_resistance: (0.5, 0.9),
            solubility: (0.0, 0.8),
            permeability: (0.5, 1.0),
            flammability: (0.6, 1.0),
            electrical_conductivity: (0.0, 0.1),
            magnetic_permeability: (0.0, 1.0),
            refractive_index: (1.0, 2.5),
            transparency: (0.0, 1.0),
            reflectivity: (0.0, 1.0),
            absorption: (0.0, 1.0),
            uv_resistance: (0.0, 1.0),

        }),
        0x06 => Some(PropRange { // Glass
            density: (2000.0, 6000.0),
            elastic_modulus: (5e10, 9e10),
            tensile_strength: (20.0, 300.0),
            compressive_strength: (200.0, 1000.0),
            hardness: (5.0, 7.0),
            fracture_toughness: (0.5, 2.0),
            fatigue_resistance: (0.1, 0.3),
            thermal_conductivity: (0.8, 2.0),
            thermal_expansion: (3e-6, 1e-5),
            melting_point: (1000.0, 1600.0),
            specific_heat: (700.0, 1500.0),
            corrosion_resistance: (0.9, 1.0),
            solubility: (0.0, 0.1),
            permeability: (0.0, 0.1),
            flammability: (0.0, 0.1),
            electrical_conductivity: (0.0, 1.0),
            magnetic_permeability: (0.0, 1.0),
            refractive_index: (1.0, 2.5),
            transparency: (0.0, 1.0),
            reflectivity: (0.0, 1.0),
            absorption: (0.0, 1.0),
            uv_resistance: (0.0, 1.0),

        }),
        0x07 => Some(PropRange { // Wood
            density: (200.0, 1200.0),
            elastic_modulus: (1e9, 2e10),
            tensile_strength: (40.0, 150.0),
            compressive_strength: (10.0, 80.0),
            hardness: (2.0, 5.0),
            fracture_toughness: (0.5, 5.0),
            fatigue_resistance: (0.3, 0.7),
            thermal_conductivity: (0.05, 0.3),
            thermal_expansion: (1e-5, 5e-5),
            melting_point: (200.0, 600.0),
            specific_heat: (1000.0, 2500.0),
            corrosion_resistance: (0.2, 0.6),
            solubility: (0.1, 0.5),
            permeability: (0.3, 0.9),
            flammability: (0.7, 1.0),
            electrical_conductivity: (0.0, 0.5),
            magnetic_permeability: (0.0, 1.0),
            refractive_index: (1.0, 2.5),
            transparency: (0.0, 1.0),
            reflectivity: (0.0, 1.0),
            absorption: (0.0, 1.0),
            uv_resistance: (0.0, 1.0),

        }),
        0x08 => Some(PropRange { // Stone
            density: (2200.0, 3000.0),
            elastic_modulus: (3e10, 7e10),
            tensile_strength: (5.0, 25.0),
            compressive_strength: (30.0, 300.0),
            hardness: (4.0, 8.0),
            fracture_toughness: (1.0, 5.0),
            fatigue_resistance: (0.1, 0.4),
            thermal_conductivity: (1.5, 5.0),
            thermal_expansion: (4e-6, 1.2e-5),
            melting_point: (800.0, 1800.0),
            specific_heat: (700.0, 1500.0),
            corrosion_resistance: (0.8, 1.0),
            solubility: (0.0, 0.01),
            permeability: (0.0, 0.2),
            flammability: (0.0, 0.1),
            electrical_conductivity: (0.0, 1.0),
            magnetic_permeability: (0.0, 1.0),
            refractive_index: (1.0, 2.5),
            transparency: (0.0, 1.0),
            reflectivity: (0.0, 1.0),
            absorption: (0.0, 1.0),
            uv_resistance: (0.0, 1.0),

        }),
        0x09 => Some(PropRange { // Concrete
            density: (2200.0, 2600.0),
            elastic_modulus: (2e10, 5e10),
            tensile_strength: (2.0, 10.0),
            compressive_strength: (20.0, 80.0),
            hardness: (3.0, 6.0),
            fracture_toughness: (0.2, 2.0),
            fatigue_resistance: (0.2, 0.5),
            thermal_conductivity: (0.8, 2.0),
            thermal_expansion: (6e-6, 1.2e-5),
            melting_point: (1000.0, 1600.0),
            specific_heat: (800.0, 1500.0),
            corrosion_resistance: (0.7, 0.95),
            solubility: (0.0, 0.05),
            permeability: (0.1, 0.5),
            flammability: (0.0, 0.1),
            electrical_conductivity: (0.0, 0.5),
            magnetic_permeability: (0.0, 1.0),
            refractive_index: (1.0, 2.5),
            transparency: (0.0, 1.0),
            reflectivity: (0.0, 1.0),
            absorption: (0.0, 1.0),
            uv_resistance: (0.0, 1.0),

        }),
        0x0A => Some(PropRange { // Composite (e.g. carbon fiber, laminates)
            density: (1200.0, 2000.0),
            elastic_modulus: (5e9, 2e11),
            tensile_strength: (200.0, 2500.0),
            compressive_strength: (100.0, 1500.0),
            hardness: (3.0, 7.0),
            fracture_toughness: (5.0, 40.0),
            fatigue_resistance: (0.5, 0.9),
            thermal_conductivity: (0.1, 20.0),
            thermal_expansion: (1e-6, 3e-5),
            melting_point: (200.0, 1200.0),
            specific_heat: (300.0, 1500.0),
            corrosion_resistance: (0.5, 0.95),
            solubility: (0.0, 0.1),
            permeability: (0.0, 0.2),
            flammability: (0.2, 0.8),
            electrical_conductivity: (0.0, 1e6),
            magnetic_permeability: (0.0, 10.0),
            refractive_index: (1.0, 2.5),
            transparency: (0.0, 1.0),
            reflectivity: (0.0, 1.0),
            absorption: (0.0, 1.0),
            uv_resistance: (0.0, 1.0),

        }),
        0x0B => Some(PropRange { // Rubber / Elastomer
            density: (900.0, 1300.0),
            elastic_modulus: (1e6, 1e8),
            tensile_strength: (5.0, 50.0),
            compressive_strength: (2.0, 30.0),
            hardness: (0.5, 3.0),
            fracture_toughness: (1.0, 10.0),
            fatigue_resistance: (0.6, 0.95),
            thermal_conductivity: (0.1, 0.5),
            thermal_expansion: (1e-4, 6e-4),
            melting_point: (200.0, 600.0),
            specific_heat: (1000.0, 3000.0),
            corrosion_resistance: (0.4, 0.8),
            solubility: (0.1, 0.9),
            permeability: (0.4, 1.0),
            flammability: (0.6, 1.0),
            electrical_conductivity: (0.0, 1.0),
            magnetic_permeability: (0.0, 1.0),
            refractive_index: (1.0, 2.5),
            transparency: (0.0, 1.0),
            reflectivity: (0.0, 1.0),
            absorption: (0.0, 1.0),
            uv_resistance: (0.0, 1.0),

        }),
        0x0C => Some(PropRange { // Liquid (generic, e.g. water, oil)
            density: (500.0, 2000.0),
            elastic_modulus: (1e3, 1e6), // bulk modulus approx
            tensile_strength: (0.0, 0.1),
            compressive_strength: (0.0, 0.1),
            hardness: (0.0, 0.1),
            fracture_toughness: (0.0, 0.1),
            fatigue_resistance: (0.0, 0.1),
            thermal_conductivity: (0.05, 1.0),
            thermal_expansion: (1e-4, 1e-3),
            melting_point: (-200.0, 500.0),
            specific_heat: (1000.0, 4000.0),
            corrosion_resistance: (0.0, 1.0),
            solubility: (0.5, 1.0),
            permeability: (1.0, 1.1),
            flammability: (0.0, 1.0),
            electrical_conductivity: (0.0, 1e4),
            magnetic_permeability: (0.0, 1.0),
            refractive_index: (1.0, 2.5),
            transparency: (0.0, 1.0),
            reflectivity: (0.0, 1.0),
            absorption: (0.0, 1.0),
            uv_resistance: (0.0, 1.0),

        }),
        0x0D => Some(PropRange { // Gas
            density: (0.1, 10.0),
            elastic_modulus: (1e2, 1e6), // compressibility
            tensile_strength: (0.0, 0.1),
            compressive_strength: (0.0, 0.1),
            hardness: (0.0, 0.1),
            fracture_toughness: (0.0, 0.1),
            fatigue_resistance: (0.0, 0.1),
            thermal_conductivity: (0.01, 0.5),
            thermal_expansion: (1e-3, 1e-2),
            melting_point: (-270.0, 0.0),
            specific_heat: (1000.0, 5000.0),
            corrosion_resistance: (0.0, 1.0),
            solubility: (0.0, 1.0),
            permeability: (1.0, 1.1),
            flammability: (0.0, 1.0),
            electrical_conductivity: (0.0, 1.0),
            magnetic_permeability: (0.0, 1.0),
            refractive_index: (1.0, 2.5),
            transparency: (0.0, 1.0),
            reflectivity: (0.0, 1.0),
            absorption: (0.0, 1.0),
            uv_resistance: (0.0, 1.0),

        }),
        0x0E => Some(PropRange { // Soil / Earth
            density: (1200.0, 2200.0),
            elastic_modulus: (1e7, 1e9),
            tensile_strength: (0.1, 5.0),
            compressive_strength: (0.5, 50.0),
            hardness: (0.5, 3.0),
            fracture_toughness: (0.1, 2.0),
            fatigue_resistance: (0.1, 0.3),
            thermal_conductivity: (0.2, 2.0),
            thermal_expansion: (1e-5, 5e-5),
            melting_point: (0.0, 1200.0),
            specific_heat: (800.0, 2000.0),
            corrosion_resistance: (0.3, 0.8),
            solubility: (0.0, 0.3),
            permeability: (0.2, 0.9),
            flammability: (0.0, 0.2),
            electrical_conductivity: (0.0, 100.0),
            magnetic_permeability: (0.0, 10.0),
            refractive_index: (1.0, 2.5),
            transparency: (0.0, 1.0),
            reflectivity: (0.0, 1.0),
            absorption: (0.0, 1.0),
            uv_resistance: (0.0, 1.0),

        }),
        0x0F => Some(PropRange { // Textile / Fabric
            density: (100.0, 1500.0),
            elastic_modulus: (1e6, 1e9),
            tensile_strength: (5.0, 100.0),
            compressive_strength: (0.5, 20.0),
            hardness: (0.5, 2.0),
            fracture_toughness: (0.5, 5.0),
            fatigue_resistance: (0.3, 0.8),
            thermal_conductivity: (0.01, 0.2),
            thermal_expansion: (5e-5, 5e-4),
            melting_point: (100.0, 600.0),
            specific_heat: (1000.0, 3000.0),
            corrosion_resistance: (0.4, 0.8),
            solubility: (0.0, 0.6),
            permeability: (0.5, 1.0),
            flammability: (0.6, 1.0),
            electrical_conductivity: (0.0, 1.0),
            magnetic_permeability: (0.0, 1.0),
            refractive_index: (1.0, 2.5),
            transparency: (0.0, 1.0),
            reflectivity: (0.0, 1.0),
            absorption: (0.0, 1.0),
            uv_resistance: (0.0, 1.0),

        }),
        0x10 => Some(PropRange { // Biological Tissue
            density: (900.0, 1200.0),
            elastic_modulus: (1e5, 1e7),
            tensile_strength: (1.0, 50.0),
            compressive_strength: (0.5, 20.0),
            hardness: (0.1, 2.0),
            fracture_toughness: (0.2, 2.0),
            fatigue_resistance: (0.3, 0.7),
            thermal_conductivity: (0.2, 0.6),
            thermal_expansion: (5e-5, 5e-4),
            melting_point: (0.0, 100.0),
            specific_heat: (2000.0, 4000.0),
            corrosion_resistance: (0.2, 0.7),
            solubility: (0.5, 1.0),
            permeability: (0.5, 1.0),
            flammability: (0.5, 0.9),
            electrical_conductivity: (0.0, 10.0),
            magnetic_permeability: (0.0, 1.0),
            refractive_index: (1.0, 2.5),
            transparency: (0.0, 1.0),
            reflectivity: (0.0, 1.0),
            absorption: (0.0, 1.0),
            uv_resistance: (0.0, 1.0),

        }),
        0x11 => Some(PropRange { // Ice
            density: (900.0, 1000.0),
            elastic_modulus: (1e9, 1e10),
            tensile_strength: (0.5, 3.0),
            compressive_strength: (5.0, 25.0),
            hardness: (1.0, 3.0),
            fracture_toughness: (0.2, 1.5),
            fatigue_resistance: (0.2, 0.5),
            thermal_conductivity: (2.0, 5.0),
            thermal_expansion: (5e-5, 1e-4),
            melting_point: (-10.0, 0.0),
            specific_heat: (500.0, 2000.0),
            corrosion_resistance: (0.8, 1.0),
            solubility: (0.0, 0.1),
            permeability: (0.0, 0.2),
            flammability: (0.0, 0.1),
            electrical_conductivity: (0.0, 0.5),
            magnetic_permeability: (0.0, 1.0),
            refractive_index: (1.0, 2.5),
            transparency: (0.0, 1.0),
            reflectivity: (0.0, 1.0),
            absorption: (0.0, 1.0),
            uv_resistance: (0.0, 1.0),

        }),
        0x12 => Some(PropRange { // Exotic (e.g. superconductors, special alloys)
            density: (3000.0, 12000.0),
            elastic_modulus: (1e11, 5e11),
            tensile_strength: (500.0, 5000.0),
            compressive_strength: (500.0, 8000.0),
            hardness: (7.0, 10.0),
            fracture_toughness: (20.0, 200.0),
            fatigue_resistance: (0.7, 1.0),
            thermal_conductivity: (10.0, 1000.0),
            thermal_expansion: (0.0, 5e-6),
            melting_point: (1000.0, 5000.0),
            specific_heat: (500.0, 2000.0),
            corrosion_resistance: (0.5, 1.0),
            solubility: (0.0, 0.01),
            permeability: (0.0, 0.1),
            flammability: (0.0, 0.1),
            electrical_conductivity: (1e6, 1e8),
            magnetic_permeability: (0.0, 10000.0),
            refractive_index: (1.0, 2.5),
            transparency: (0.0, 1.0),
            reflectivity: (0.0, 1.0),
            absorption: (0.0, 1.0),
            uv_resistance: (0.0, 1.0),

        }),

        _ => None,
    }
}



/// Generate props constrained by category ranges
pub fn generate_props_from_category(cat: u8, rng: &mut impl Rng) -> Option<MatProps> {
    get_category_ranges(cat).map(|r| MatProps {
        density: rng.random_range(r.density.0..r.density.1),
        elastic_modulus: rng.random_range(r.elastic_modulus.0..r.elastic_modulus.1),
        tensile_strength: rng.random_range(r.tensile_strength.0..r.tensile_strength.1),
        compressive_strength: rng.random_range(r.compressive_strength.0..r.compressive_strength.1),
        hardness: rng.random_range(r.hardness.0..r.hardness.1),
        fracture_toughness: rng.random_range(r.fracture_toughness.0..r.fracture_toughness.1),
        fatigue_resistance: rng.random_range(r.fatigue_resistance.0..r.fatigue_resistance.1),
        thermal_conductivity: rng.random_range(r.thermal_conductivity.0..r.thermal_conductivity.1),
        thermal_expansion: rng.random_range(r.thermal_expansion.0..r.thermal_expansion.1),
        melting_point: rng.random_range(r.melting_point.0..r.melting_point.1),
        specific_heat: rng.random_range(r.specific_heat.0..r.specific_heat.1),
        corrosion_resistance: rng.random_range(r.corrosion_resistance.0..r.corrosion_resistance.1),
        solubility: rng.random_range(r.solubility.0..r.solubility.1),
        permeability: rng.random_range(r.permeability.0..r.permeability.1),
        flammability: rng.random_range(r.flammability.0..r.flammability.1),
        electrical_conductivity: rng.random_range(r.electrical_conductivity.0..r.electrical_conductivity.1),
        magnetic_permeability: rng.random_range(r.magnetic_permeability.0..r.magnetic_permeability.1),
        refractive_index: rng.random_range(r.refractive_index.0..r.refractive_index.1),
        transparency: rng.random_range(r.transparency.0..r.transparency.1),
        reflectivity: rng.random_range(r.reflectivity.0..r.reflectivity.1),
        absorption: rng.random_range(r.absorption.0..r.absorption.1),
        uv_resistance: rng.random_range(r.uv_resistance.0..r.uv_resistance.1),  

    })
}

pub fn generate_props_for_material(cat: u8, variant: u16, grade: u16) -> MatProps {
    let mut rng = rand::thread_rng();

    // Step 1: Base range by category
    let base = get_category_ranges(cat).expect("invalid material category");

    // Step 2: Apply variant bias (e.g., copper vs steel within 'metal')
    // Variant shifts the midpoint slightly ±10%
    let bias_factor = (variant as f32 / 65535.0) * 0.2 - 0.1;

    // Step 3: Grade tightens range — higher grade = narrower band
    let grade_factor = 1.0 - (grade as f32 / 65535.0) * 0.8; // 0.2–1.0 scaling

    // Step 4: Generate around the midpoint with slight random deviation
    fn jitter(base: (f32, f32), bias: f32, tightness: f32, rng: &mut impl rand::Rng) -> f32 {
        let mid = (base.0 + base.1) / 2.0;
        let span = (base.1 - base.0) * tightness * 0.5;
        let val = mid * (1.0 + bias) + rng.gen_range(-span..span);
        val.clamp(base.0, base.1)
    }

let mut props = MatProps {
    density: jitter(base.density, bias_factor, grade_factor, &mut rng),
    elastic_modulus: jitter(base.elastic_modulus, bias_factor, grade_factor, &mut rng),
    tensile_strength: jitter(base.tensile_strength, bias_factor, grade_factor, &mut rng),
    compressive_strength: jitter(base.compressive_strength, bias_factor, grade_factor, &mut rng),
    hardness: jitter(base.hardness, bias_factor, grade_factor, &mut rng),
    fracture_toughness: jitter(base.fracture_toughness, bias_factor, grade_factor, &mut rng),
    fatigue_resistance: jitter(base.fatigue_resistance, bias_factor, grade_factor, &mut rng),
    thermal_conductivity: jitter(base.thermal_conductivity, bias_factor, grade_factor, &mut rng),
    thermal_expansion: jitter(base.thermal_expansion, bias_factor, grade_factor, &mut rng),
    melting_point: jitter(base.melting_point, bias_factor, grade_factor, &mut rng),
    specific_heat: jitter(base.specific_heat, bias_factor, grade_factor, &mut rng),

    corrosion_resistance: jitter(base.corrosion_resistance, bias_factor, grade_factor, &mut rng),
    solubility: jitter(base.solubility, bias_factor, grade_factor, &mut rng),
    permeability: jitter(base.permeability, bias_factor, grade_factor, &mut rng),
    flammability: jitter(base.flammability, bias_factor, grade_factor, &mut rng),
    electrical_conductivity: jitter(base.electrical_conductivity, bias_factor, grade_factor, &mut rng),
    magnetic_permeability: jitter(base.magnetic_permeability, bias_factor, grade_factor, &mut rng),

    // 🔧 Fix: these should use `base`, not `r`
    refractive_index: jitter(base.refractive_index, bias_factor, grade_factor, &mut rng),
    transparency: jitter(base.transparency, bias_factor, grade_factor, &mut rng),
    reflectivity: jitter(base.reflectivity, bias_factor, grade_factor, &mut rng),
    absorption: jitter(base.absorption, bias_factor, grade_factor, &mut rng),
    uv_resistance: jitter(base.uv_resistance, bias_factor, grade_factor, &mut rng),
};


    normalize_props(&mut props);
    props
}
/// Normalize numeric properties to bring all values into expected ranges.
/// Useful for clamping noisy random data.
fn normalize_props(props: &mut crate::matcat::materials::MatProps) {
    use num_traits::clamp;

    props.density = clamp(props.density, 0.0, 2.5e4);
    props.elastic_modulus = clamp(props.elastic_modulus, 0.0, 1e12);
    props.tensile_strength = clamp(props.tensile_strength, 0.0, 1e4);
    props.compressive_strength = clamp(props.compressive_strength, 0.0, 1e4);
    props.hardness = clamp(props.hardness, 0.0, 10.0);
    props.fracture_toughness = clamp(props.fracture_toughness, 0.0, 300.0);
    props.fatigue_resistance = clamp(props.fatigue_resistance, 0.0, 1.0);
    props.thermal_conductivity = clamp(props.thermal_conductivity, 0.0, 2e3);
    props.thermal_expansion = clamp(props.thermal_expansion, 0.0, 1e-2);
    props.melting_point = clamp(props.melting_point, -300.0, 7000.0);
    props.specific_heat = clamp(props.specific_heat, 100.0, 5000.0);

    props.corrosion_resistance = clamp(props.corrosion_resistance, 0.0, 1.0);
    props.solubility = clamp(props.solubility, 0.0, 1.0);
    props.permeability = clamp(props.permeability, 0.0, 2.0);
    props.flammability = clamp(props.flammability, 0.0, 1.0);
    props.electrical_conductivity = clamp(props.electrical_conductivity, 0.0, 1e8);
    props.magnetic_permeability = clamp(props.magnetic_permeability, 0.0, 1e6);
    props.refractive_index = clamp(props.refractive_index, 1.0, 3.0);
    props.transparency = clamp(props.transparency, 0.0, 1.0);
    props.reflectivity = clamp(props.reflectivity, 0.0, 1.0);
    props.absorption = clamp(props.absorption, 0.0, 1.0);
    props.uv_resistance = clamp(props.uv_resistance, 0.0, 1.0);
    
    
}
