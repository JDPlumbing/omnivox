use serde::Serialize;

use crate::core::objex::matcat::{
    categories::{CategoryId, CATEGORY_MAP},
    variants::{VariantId, VARIANT_MAP},
    grades::{GradeId, GRADE_MAP},
    materials::{MatCatId, props_for},
    properties::MatProps,
};

/// Generic id + name pair for UI dropdowns
#[derive(Debug, Serialize)]
pub struct IdName {
    pub id: u32,
    pub name: &'static str,
}

/// Resolve result for UI → engine handoff
#[derive(Debug, Serialize)]
pub struct ResolveResponse {
    pub category: u8,
    pub variant: Option<u16>,
    pub grade: Option<u16>,
    pub name: String,
}

/// Preview properties without creating an Objex
#[derive(Debug, Serialize)]
pub struct PreviewResponse {
    pub matcat_id: String,
    pub properties: MatProps,
}

/// -------------------------
/// Categories
/// -------------------------

pub fn get_categories() -> Vec<IdName> {
    CATEGORY_MAP
        .iter()
        .map(|(id, name)| IdName {
            id: id.0 as u32,
            name,
        })
        .collect()
}

/// -------------------------
/// Variants
/// -------------------------

pub fn get_variants(category: u8) -> Vec<IdName> {
    let cat = CategoryId(category);

    VARIANT_MAP
        .iter()
        .filter(|((c, _), _)| *c == cat)
        .map(|((_, v), name)| IdName {
            id: v.0 as u32,
            name,
        })
        .collect()
}

/// -------------------------
/// Grades
/// -------------------------

pub fn get_grades(category: u8, variant: u16) -> Vec<IdName> {
    let cat = CategoryId(category);
    let var = VariantId(variant);

    GRADE_MAP
        .iter()
        .filter(|((c, v, _), _)| *c == cat && *v == var)
        .map(|((_, _, g), name)| IdName {
            id: g.0 as u32,
            name,
        })
        .collect()
}

/// -------------------------
/// Resolve
/// -------------------------

pub fn resolve_material(
    category: u8,
    variant: Option<u16>,
    grade: Option<u16>,
) -> ResolveResponse {
    let matcat = MatCatId::new(
        category,
        variant.unwrap_or(0),
        grade.unwrap_or(0),
    );

    ResolveResponse {
        category,
        variant,
        grade,
        name: matcat.name(),
    }
}

/// -------------------------
/// Preview
/// -------------------------

pub fn preview_material(
    category: u8,
    variant: Option<u16>,
    grade: Option<u16>,
) -> PreviewResponse {
    let matcat = MatCatId::new(
        category,
        variant.unwrap_or(0),
        grade.unwrap_or(0),
    );

    let props = props_for(&matcat);

    PreviewResponse {
        matcat_id: format!(
            "{:02}-{:04}-{:04}",
            category,
            variant.unwrap_or(0),
            grade.unwrap_or(0)
        ),
        properties: props,
    }
}
