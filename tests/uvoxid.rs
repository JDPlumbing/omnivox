use omnivox::core::uvoxid::{
    UvoxId, RUm, LatCode, LonCode, Delta, DRUm, DLat, DLon
};

#[test]
fn test_basic_construction() {
    let id = UvoxId::new(
        RUm(6_371_000_000_000),
        LatCode(0),
        LonCode(0),
    );
    assert_eq!(id.lat_code.0, 0);
}

#[test]
fn test_delta_application() {
    let mut id = UvoxId::new(
        RUm(6_371_000_000_000),
        LatCode(0),
        LonCode(0),
    );

    let delta = Delta::new(10, 1000, -500);
    id.apply_delta(delta);

    assert_eq!(id.r_um.0, 6_371_000_000_010);
}
