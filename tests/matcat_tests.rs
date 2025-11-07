#[test]
fn test_deterministic_props() {
    use crate::matcat::materials::{MatCatId, props_for};

    let id = MatCatId::new(1, 2, 1);
    let a = props_for(&id);
    let b = props_for(&id);
    assert_eq!(a.density, b.density);
    assert_eq!(a.elastic_modulus, b.elastic_modulus);
}
