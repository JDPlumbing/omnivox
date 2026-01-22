pub fn rotate_around_axis(v: [f64; 3], axis: [f64; 3], angle: f64) -> [f64; 3] {
    let cos_a = angle.cos();
    let sin_a = angle.sin();

    let cross = [
        axis[1]*v[2] - axis[2]*v[1],
        axis[2]*v[0] - axis[0]*v[2],
        axis[0]*v[1] - axis[1]*v[0],
    ];

    let dot = axis[0]*v[0] + axis[1]*v[1] + axis[2]*v[2];

    [
        v[0]*cos_a + cross[0]*sin_a + axis[0]*dot*(1.0 - cos_a),
        v[1]*cos_a + cross[1]*sin_a + axis[1]*dot*(1.0 - cos_a),
        v[2]*cos_a + cross[2]*sin_a + axis[2]*dot*(1.0 - cos_a),
    ]
}
