use crate::objex::systems::mechanical::MechanicalProps;
use crate::objex::core::CompositeObject;
use crate::geospec::{Dimensions, Volume, SurfaceArea};

/// Derive mechanical properties for a composite object.
/// - Young’s modulus → weighted average by volume
/// - Hardness → weakest layer
/// - Fracture toughness → weakest layer
/// - Inertia → sum of all layers
pub fn derive_mechanical_composite<T: Dimensions + Volume + SurfaceArea>(
    obj: &CompositeObject<T>
) -> MechanicalProps {
    let mut total_vol = 0.0;
    let mut weighted_modulus = 0.0;
    let mut min_hardness = f32::INFINITY;
    let mut min_fracture = f32::INFINITY;
    let mut total_inertia = 0.0;

    for layer in &obj.layers {
        let vol = layer.shape.volume();
        let area = layer.shape.surface_area();
        let mass = vol * layer.material.density as f64;
        let char_len = (vol / area).abs().max(1e-6);

        weighted_modulus += vol * layer.material.elastic_modulus as f64; // <-- cast here
        total_vol += vol;

        min_hardness = min_hardness.min(layer.material.hardness);
        min_fracture = min_fracture.min(layer.material.fracture_toughness);

        total_inertia += mass * char_len.powi(2);
    }

    MechanicalProps {
        youngs_modulus: (weighted_modulus / total_vol) as f32,
        hardness: min_hardness,
        fracture_toughness: min_fracture,
        inertia: total_inertia,
    }
}
