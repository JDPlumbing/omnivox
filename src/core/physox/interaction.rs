//! Material interactions: restitution, damage, and energy exchange.

use crate::core::objex::matcat::properties::MatProps;

/// Calculate combined restitution (bounciness) between two materials.
pub fn restitution(a: &MatProps, b: &MatProps) -> f32 {
    let h_factor = (a.hardness + b.hardness) / 20.0; // normalize 0–1
    let e_factor = ((a.elastic_modulus + b.elastic_modulus) / 8.0e11) as f32;
    (h_factor * e_factor).clamp(0.0, 1.0)
}

/// Estimate damage ratio based on impact energy vs. fracture toughness.
pub fn damage(a: &MatProps, b: &MatProps, impact_energy: f32) -> f32 {
    let toughness = (a.fracture_toughness + b.fracture_toughness) / 2.0;
    (impact_energy / toughness).clamp(0.0, 1.0)
}
