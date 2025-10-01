/// A difference between two UvoxIds
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Delta {
    pub dr_um: i64,
    pub dlat: i64,
    pub dlon: i64,
}

impl Delta {
    pub fn new(dr_um: i64, dlat: i64, dlon: i64) -> Self {
        Self { dr_um, dlat, dlon }
    }
}
