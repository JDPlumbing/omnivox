use std::ops::Mul;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat3 {
    pub m: [[f64; 3]; 3],
}

impl Mat3 {
    pub fn identity() -> Self {
        Self {
            m: [
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn rotation_x(theta: f64) -> Self {
        let (s, c) = theta.sin_cos();
        Self {
            m: [
                [1.0, 0.0, 0.0],
                [0.0,  c, -s],
                [0.0,  s,  c],
            ],
        }
    }

    pub fn rotation_y(theta: f64) -> Self {
        let (s, c) = theta.sin_cos();
        Self {
            m: [
                [ c, 0.0, s],
                [0.0, 1.0, 0.0],
                [-s, 0.0, c],
            ],
        }
    }

    pub fn rotation_z(theta: f64) -> Self {
        let (s, c) = theta.sin_cos();
        Self {
            m: [
                [ c, -s, 0.0],
                [ s,  c, 0.0],
                [0.0, 0.0, 1.0],
            ],
        }
    }
}

impl Mul<[f64; 3]> for Mat3 {
    type Output = [f64; 3];

    fn mul(self, v: [f64; 3]) -> [f64; 3] {
        [
            self.m[0][0] * v[0] + self.m[0][1] * v[1] + self.m[0][2] * v[2],
            self.m[1][0] * v[0] + self.m[1][1] * v[1] + self.m[1][2] * v[2],
            self.m[2][0] * v[0] + self.m[2][1] * v[1] + self.m[2][2] * v[2],
        ]
    }
}

impl Mul for Mat3 {
    type Output = Mat3;

    fn mul(self, rhs: Mat3) -> Mat3 {
        let mut r = [[0.0; 3]; 3];

        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    r[i][j] += self.m[i][k] * rhs.m[k][j];
                }
            }
        }

        Mat3 { m: r }
    }
}




















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
