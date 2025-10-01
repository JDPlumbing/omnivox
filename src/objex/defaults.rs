use crate::geospec::shapes::*;
use crate::objex::{Objex, Shape, MaterialLink};
use crate::objex::builder::ObjexBuilder;

impl Objex {
    pub fn unit_point() -> Self {
        ObjexBuilder::new()
            .name("point")
            .shape(Shape::Point(Point))
            .material(MaterialLink::vacuum())
            .build()
    }

    pub fn unit_cube() -> Self {
        ObjexBuilder::new()
            .name("unit cube")
            .shape(Shape::Box(BoxShape {
                length: 1.0,
                width: 1.0,
                height: 1.0,
            }))
            .material(MaterialLink::vacuum())
            .build()
    }

    pub fn unit_sphere() -> Self {
        ObjexBuilder::new()
            .name("unit sphere")
            .shape(Shape::Sphere(Sphere { radius: 1.0 }))
            .material(MaterialLink::vacuum())
            .build()
    }

    pub fn unit_cylinder() -> Self {
        ObjexBuilder::new()
            .name("unit cylinder")
            .shape(Shape::Cylinder(Cylinder {
                radius: 1.0,
                height: 1.0,
            }))
            .material(MaterialLink::vacuum())
            .build()
    }
}
