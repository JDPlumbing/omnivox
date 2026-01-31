use crate::core::physics::units::{
    length::Meters,
    pressure::Pascals,
};

/// Compute pressure at a given altitude using the barometric formula
/// for an isothermal atmosphere.
///
/// P(z) = P0 * exp(-z / H)
pub fn pressure_at_altitude(
    surface_pressure: Pascals,
    scale_height: Meters,
    altitude: Meters,
) -> Pascals {
    if altitude.0 <= 0.0 {
        return surface_pressure;
    }

    let exponent = -altitude.0 / scale_height.0;
    Pascals(surface_pressure.0 * exponent.exp())
}
