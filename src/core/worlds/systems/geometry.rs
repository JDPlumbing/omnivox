use crate::core::physics::units::angle::Radians;
use crate::core::math::vec3::Vec3;

pub fn surface_normal_from_lat_lon(
    lat: Radians,
    lon: Radians,
) -> Vec3 {
    let (sin_lat, cos_lat) = lat.0.sin_cos();
    let (sin_lon, cos_lon) = lon.0.sin_cos();

    Vec3::new(
        cos_lat * cos_lon,
        cos_lat * sin_lon,
        sin_lat,
    )
}
