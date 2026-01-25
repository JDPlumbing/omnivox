

world conceputal model:;

WorldFrame:{
World:{
    WolrdId,
    Name,
    Description,
    world_epoch,
    WorldEnvironemnt {
        space: WorldSpace,
        fields: [],
    }
    WorldEnvDescriptor: {
        space: WorldSpace:{
                   surface_radius_m,
                   up_model:UpModel:{Radial|Axial}},
        gravity: GravityModel{:
                      kind: GravityKind:{Radial|Uniform|None}
                      strength: f64,
                    },
        medium: MediumModel:{defualt: Medium: {Vacuum|Gas|Liquid|Solid}},
        land: LandModel:{Flat|Noise},
        atmosphere: AtmosphereModel: {
                sea_level_density,
                scale_height_m,
                max_height_m,
        },
        temperature: TemperatureModel{
            surface_temp_k,
            lapse_rate_k_per_m,
        },
        pressure:PressureModel{
            derive_from_density: bool,
        },
    }
}
Parent:World,}