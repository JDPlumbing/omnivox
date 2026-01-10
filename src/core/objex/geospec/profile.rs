use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Profile {
    Circle {
        outer_radius: f64,
        inner_radius: Option<f64>,
    },
    Rect {
        width: f64,
        height: f64,
        wall: Option<f64>,
    },
    IBeam {
        flange_width: f64,
        flange_thickness: f64,
        web_height: f64,
        web_thickness: f64,
    },
}

pub trait ProfileArea {
    fn area(&self) -> f64;
}

pub trait ProfilePerimeter {
    fn perimeter(&self) -> f64;
}


impl ProfileArea for Profile {
    fn area(&self) -> f64 {
        match self {
            Profile::Circle { outer_radius, inner_radius } => {
                let outer = std::f64::consts::PI * outer_radius.powi(2);
                let inner = inner_radius
                    .map(|r| std::f64::consts::PI * r.powi(2))
                    .unwrap_or(0.0);
                outer - inner
            }

            Profile::Rect { width, height, wall } => {
                match wall {
                    None => width * height,
                    Some(w) => {
                        let inner_w = width - 2.0 * w;
                        let inner_h = height - 2.0 * w;
                        width * height - inner_w.max(0.0) * inner_h.max(0.0)
                    }
                }
            }

            Profile::IBeam {
                flange_width,
                flange_thickness,
                web_height,
                web_thickness,
            } => {
                let flange_area = flange_width * flange_thickness * 2.0;
                let web_area = web_thickness * web_height;
                flange_area + web_area
            }
        }
    }
}

impl ProfilePerimeter for Profile {
    fn perimeter(&self) -> f64 {
        match self {
            Profile::Circle { outer_radius, inner_radius } => {
                let outer = 2.0 * std::f64::consts::PI * outer_radius;
                let inner = inner_radius
                    .map(|r| 2.0 * std::f64::consts::PI * r)
                    .unwrap_or(0.0);
                outer + inner
            }

            Profile::Rect { width, height, wall } => {
                match wall {
                    None => 2.0 * (width + height),
                    Some(w) => {
                        let inner_w = width - 2.0 * w;
                        let inner_h = height - 2.0 * w;
                        2.0 * (width + height)
                            + 2.0 * (inner_w.max(0.0) + inner_h.max(0.0))
                    }
                }
            }

            Profile::IBeam {
                flange_width,
                flange_thickness,
                web_height,
                web_thickness,
            } => {
                // Outline perimeter only (correct for corrosion / exposure)
                let total_height = 2.0 * flange_thickness + web_height;

                2.0 * (
                    flange_width
                    + total_height
                    + web_thickness
                )
            }
        }
    }
}
