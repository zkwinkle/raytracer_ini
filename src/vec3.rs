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
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}
