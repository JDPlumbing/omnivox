/// Unit conversion utilities for uvoxid coordinates (base unit = micrometer)
/// All internal spatial coordinates and displacements use integer micrometers (µm)
/// These helpers allow easy conversion for human-readable or analytical output.

/// --- Length conversions ---
pub fn um_to_m(um: i64) -> f64 {
    um as f64 * 1e-6
}

pub fn um_to_cm(um: i64) -> f64 {
    um as f64 * 1e-4
}

pub fn um_to_mm(um: i64) -> f64 {
    um as f64 * 1e-3
}

pub fn um_to_in(um: i64) -> f64 {
    um as f64 * 3.9370079e-5 // 1 µm = 3.937e-5 inches
}

pub fn um_to_ft(um: i64) -> f64 {
    um as f64 * 3.2808399e-6 // 1 µm = 3.2808e-6 feet
}

pub fn um_to_km(um: i64) -> f64 {
    um as f64 * 1e-9
}

/// --- Reverse conversions ---
pub fn m_to_um(m: f64) -> i64 {
    (m * 1e6).round() as i64
}

pub fn cm_to_um(cm: f64) -> i64 {
    (cm * 1e4).round() as i64
}

pub fn mm_to_um(mm: f64) -> i64 {
    (mm * 1e3).round() as i64
}

pub fn in_to_um(inches: f64) -> i64 {
    (inches / 3.9370079e-5).round() as i64
}

pub fn ft_to_um(feet: f64) -> i64 {
    (feet / 3.2808399e-6).round() as i64
}

pub fn km_to_um(km: f64) -> i64 {
    (km * 1e9).round() as i64
}

/// --- Optional pretty formatting trait ---
pub trait HumanLength {
    fn to_human(&self) -> String;
}

impl HumanLength for i64 {
    fn to_human(&self) -> String {
        let abs_val = self.abs();
        if abs_val >= 1_000_000_000_000 {
            format!("{:.3} km", um_to_km(*self))
        } else if abs_val >= 1_000_000_000 {
            format!("{:.3} m", um_to_m(*self))
        } else if abs_val >= 1_000_000 {
            format!("{:.3} mm", um_to_mm(*self))
        } else {
            format!("{} µm", self)
        }
    }
}
