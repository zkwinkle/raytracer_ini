use crate::constants::TOLERANCE;
use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

macro_rules! vec3_op_num {
    ($($path:ident)::+, $fn:ident) => {
        impl $($path)::+<f64> for Vec3 {
            type Output = Vec3;
            fn $fn(self, other: f64) -> Self::Output {
                Vec3 {
                    x: self.x.$fn(other),
                    y: self.y.$fn(other),
                    z: self.z.$fn(other),
                }
            }
        }
        impl $($path)::+<Vec3> for f64 {
            type Output = Vec3;
            fn $fn(self, other: Vec3) -> Self::Output {
                Vec3 {
                    x: other.x.$fn(self),
                    y: other.y.$fn(self),
                    z: other.z.$fn(self),
                }
            }
        }
    };
}

vec3_op_num!(ops::Mul, mul);
vec3_op_num!(ops::Div, div);

macro_rules! vec3_op_vec {
    ($($path:ident)::+, $fn:ident) => {
        impl $($path)::+ for Vec3 {
            type Output = Self;
            fn $fn(self, other: Self) -> Self::Output {
                Vec3 {
                    x: self.x.$fn(other.x),
                    y: self.y.$fn(other.y),
                    z: self.z.$fn(other.z),
                }
            }
        }
    };
}

vec3_op_vec!(ops::Add, add);
vec3_op_vec!(ops::Sub, sub);

impl Vec3 {
    /// Get the 2-norm of the vector
    pub fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Normalized vector with 2-norm (magnitude of 1)
    pub fn normalize(&self) -> Vec3 {
        *self / self.norm()
    }

    /// Just a shorthand for initialization
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn dot(self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// The angle is in radians
    pub fn rotate_x(self, angle: f64) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y * angle.cos() - self.z * angle.sin(),
            z: self.y * angle.sin() + self.z * angle.cos(),
        }
    }
    /// The angle is in radians
    pub fn rotate_y(self, angle: f64) -> Vec3 {
        Vec3 {
            x: self.x * angle.cos() + self.y * angle.sin(),
            y: self.y,
            z: -1.0 * self.x * angle.sin() + self.z * angle.cos(),
        }
    }
    /// The angle is in radians
    pub fn rotate_z(self, angle: f64) -> Vec3 {
        Vec3 {
            x: self.x * angle.cos() + self.y * angle.sin(),
            y: self.x * angle.sin() - self.y * angle.cos(),
            z: self.z,
        }
    }

    pub fn to_align(self, to_align: Vec3) -> [[f64; 3]; 3] {
        const I: [[f64; 3]; 3] = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];

        // safety for unit vectors
        let a = self.normalize();
        let b = to_align.normalize();

        let c = a.dot(b);

        if (c + 1.0).abs() < TOLERANCE {
            return I;
        }

        let v = a.cross(b);

        let skew = [[0.0, -v.z, v.y], [v.z, 0.0, -v.x], [-v.y, v.x, 0.0]];

        let vxvy = v.x * v.y;
        let vxvz = v.x * v.z;
        let vyvz = v.y * v.z;
        let vx2 = v.x * v.x;
        let vy2 = v.y * v.y;
        let vz2 = v.z * v.z;

        let skew2 = [
            [-vy2 - vz2, vxvy, vxvz],
            [vxvy, -vx2 - vz2, vyvz],
            [vxvz, vyvz, -vx2 - vy2],
        ];

        matrix_sum(I, matrix_sum(skew, matrix_mul_k(skew2, 1.0 / (1.0 + c))))
    }

    pub fn apply_matrix(self, matrix: [[f64; 3]; 3]) -> Vec3 {
        let mut result: [f64; 3] = [0.0; 3];
        for i in 0..3 {
            result[i] = matrix[i][0] * self.x + matrix[i][1] * self.y + matrix[i][2] * self.z;
        }

        Vec3 {
            x: result[0],
            y: result[1],
            z: result[2],
        }
    }

    pub fn translation(self, x: f64, y: f64, z: f64) -> Vec3 {
        self + Vec3 { x, y, z }
    }
}

fn matrix_sum(a: [[f64; 3]; 3], b: [[f64; 3]; 3]) -> [[f64; 3]; 3] {
    [
        [a[0][0] + b[0][0], a[0][1] + b[0][1], a[0][2] + b[0][2]],
        [a[1][0] + b[1][0], a[1][1] + b[1][1], a[1][2] + b[1][2]],
        [a[2][0] + b[2][0], a[2][1] + b[2][1], a[2][2] + b[2][2]],
    ]
}

//fn matrix_mul(a: [[f64; 3]; 3], b: [[f64; 3]; 3]) -> [[f64; 3]; 3] {
//    let mut result: [[f64; 3]; 3] = [[0.0; 3]; 3];
//    for i in 0..3 {
//        for j in 0..3 {
//            result[i][j] = a[i][0] * b[0][j] + a[i][1] * b[1][j] + a[i][2] * b[2][j];
//        }
//    }
//    result
//}
//
fn matrix_mul_k(a: [[f64; 3]; 3], k: f64) -> [[f64; 3]; 3] {
    let mut result: [[f64; 3]; 3] = [[0.0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            result[i][j] = a[i][j] * k
        }
    }
    result
}
