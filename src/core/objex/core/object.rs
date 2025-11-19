use crate::objex::geospec::{Dimensions, Volume, SurfaceArea};
use crate::objex::matcat::{MatCatId, MatProps, props_for};

/// Core: combines a shape (geometry) with material properties
pub struct Object<T: Dimensions + Volume + SurfaceArea> {
    pub shape: T,
    pub material: MatProps,
}

impl<T: Dimensions + Volume + SurfaceArea> Object<T> {
    pub fn new(shape: T, mat_id: MatCatId) -> Self {
        let props = props_for(&mat_id);
        Self { shape, material: props }
    }

    pub fn volume(&self) -> f64 {
        self.shape.volume()
    }

    pub fn surface_area(&self) -> f64 {
        self.shape.surface_area()
    }

    pub fn density(&self) -> f32 {
        self.material.density
    }

    pub fn mass(&self) -> f64 {
        self.density() as f64 * self.volume()
    }
}

