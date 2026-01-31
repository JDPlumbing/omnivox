use crate::core::physics::constants::STEFAN_BOLTZMANN;
use crate::core::physics::units::temperature::Kelvin;
use crate::core::physics::units::irradiance::WattsPerSquareMeter;
use crate::core::physics::units::albedo::Albedo;

pub fn equilibrium_temperature(
    insolation: WattsPerSquareMeter,
    albedo: Albedo,
) -> Kelvin {
    let absorbed = (1.0 - albedo.0) * insolation.0;
    Kelvin((absorbed / STEFAN_BOLTZMANN).powf(0.25))
}
