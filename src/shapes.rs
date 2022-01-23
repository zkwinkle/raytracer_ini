use anyhow::{anyhow, Result};
use enum_dispatch::enum_dispatch;
use std::f64::consts::PI;
use std::iter::Sum;
use std::ops;

use crate::constants::TOLERANCE;
use crate::raytracer::Ray;
use crate::vec3::Vec3;

#[allow(dead_code)]
pub mod colors {
    use super::Color;

    pub const WHITE: Color = Color {
        r: 1.0,
        b: 1.0,
        g: 1.0,
    };
    pub const BLACK: Color = Color {
        r: 0.0,
        b: 0.0,
        g: 0.0,
    };
}

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

fn check_ranges<N: PartialOrd + ToString>(values: Vec<N>, min: N, max: N) -> Result<()> {
    let mut wrong_vals = values.iter().filter(|v| **v < min || **v > max).peekable();
    if wrong_vals.peek().is_some() {
        Err(
            anyhow!("Values for {} type given outside the [{}, {}] range. The following were the erronous ranges:{}", std::any::type_name::<N>(), min.to_string(), max.to_string(), wrong_vals.fold(String::from(""),|acc, v| acc + " " + &v.to_string())),
        )
    } else {
        Ok(())
    }
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Result<Color> {
        check_ranges(vec![r, g, b], 0.0, 1.0)?;
        Ok(Color { r, g, b })
    }

    pub fn from_hex(hex: &str) -> Result<Color> {
        if is_hex_format(hex) {
            Ok(Color::new(
                u8::from_str_radix(&hex[1..=2], 16)? as f64 / 255.0,
                u8::from_str_radix(&hex[3..=4], 16)? as f64 / 255.0,
                u8::from_str_radix(&hex[5..=6], 16)? as f64 / 255.0,
            )?)
        } else {
            Err(anyhow!(
                "from_hex() llamado en string incorrectamente formateado para hexadecimales: '{}'",
                hex
            ))
        }
    }

    pub fn min(self, min_val: f64) -> Color {
        Self {
            r: self.r.min(min_val),
            g: self.g.min(min_val),
            b: self.b.min(min_val),
        }
    }
}

impl ops::Mul<f64> for Color {
    type Output = Color;
    fn mul(self, other: f64) -> Self::Output {
        Color {
            r: self.r.mul(other),
            g: self.g.mul(other),
            b: self.b.mul(other),
        }
    }
}

impl ops::Mul<Color> for f64 {
    type Output = Color;
    fn mul(self, other: Color) -> Self::Output {
        Color {
            r: other.r.mul(self),
            g: other.g.mul(self),
            b: other.b.mul(self),
        }
    }
}

macro_rules! color_op_vec {
    ($($path:ident)::+, $fn:ident) => {
        impl $($path)::+ for Color {
            type Output = Self;
            fn $fn(self, other: Self) -> Self::Output {
                Color {
                    r: self.r.$fn(other.r).clamp(0.0, 1.0),
                    g: self.g.$fn(other.g).clamp(0.0, 1.0),
                    b: self.b.$fn(other.b).clamp(0.0, 1.0),
                }
            }
        }
    };
}

color_op_vec!(ops::Add, add);
color_op_vec!(ops::Sub, sub);
color_op_vec!(ops::Mul, mul);

impl Sum<Self> for Color {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(colors::BLACK, |acc, c| acc + c)
    }
}

fn is_hex_format(hex: &str) -> bool {
    hex.starts_with('#') && hex.len() == 7 && hex[1..].chars().all(|d| d.is_digit(16))
}

#[derive(Clone, Debug)]
pub struct Plane {
    normal: Vec3,
    anchor: Vec3,
    params: ObjectParameters,
}

impl Plane {
    pub fn new(normal: Vec3, point: Vec3, params: ObjectParameters) -> Plane {
        //let d = -1.0 * (normal.x * point.x + normal.y * point.y + normal.z * point.z);
        //let d = d / normal.norm();
        Plane {
            anchor: point,
            normal: normal.normalize(),
            params,
        }
    }
}

impl ShapeCalculations for Plane {
    /// Returns the distance "t" from the camera to the point
    fn get_intersection(&self, ray: &Ray) -> Option<f64> {
        let normal = self.normal;
        let denominator = normal.dot(ray.dir);

        if denominator.abs() < TOLERANCE {
            None
        } else {
            let t = 1.0 * (self.anchor - ray.anchor).dot(normal) / denominator;
            // Check it's in front of camera
            if t > 0.0 {
                Some(t)
            } else {
                None
            }
        }
    }

    fn get_normal_vec(&self, _: Vec3) -> Vec3 {
        self.normal
    }

    fn get_texture_coords(&self, intersection: Vec3) -> TextureCoords {
        let mut x_axis = self.normal.cross(Vec3::new(0.0, 0.0, 1.0));
        if x_axis.norm() == 0.0 {
            x_axis = self.normal.cross(Vec3::new(0.0, 1.0, 0.0));
        }
        let y_axis = self.normal.cross(x_axis);

        let plane_vec = intersection - self.anchor;

        TextureCoords {
            x: plane_vec.dot(x_axis) as f64,
            y: plane_vec.dot(y_axis) as f64,
        }
    }

    fn get_params(&self) -> &ObjectParameters {
        &self.params
    }
}

#[derive(Clone, Debug)]
pub struct Sphere {
    center: Vec3,
    r: f64,
    params: ObjectParameters,
}

impl Sphere {
    pub fn new(center: Vec3, r: f64, params: ObjectParameters) -> Sphere {
        Sphere { center, r, params }
    }
}

impl ShapeCalculations for Sphere {
    /// Returns the distance "t" from the camera to the point
    fn get_intersection(&self, ray: &Ray) -> Option<f64> {
        let anchor = ray.anchor;
        let dir = ray.dir;
        let center = self.center;

        let b = 2.0
            * (dir.x * (anchor.x - center.x)
                + dir.y * (anchor.y - center.y)
                + dir.z * (anchor.z - center.z));

        let c = (anchor.x - center.x).powi(2)
            + (anchor.y - center.y).powi(2)
            + (anchor.z - center.z).powi(2)
            - (self.r * self.r);

        let determinant = (b * b - 4.0 * c).sqrt();

        if determinant.is_nan() {
            None
        } else {
            let t1 = (-b - determinant) / 2.0;
            let t2 = (-b + determinant) / 2.0;
            if t1 > 0.0 {
                Some(t1)
            } else if t2 < 0.0 {
                None
            } else {
                Some(t2)
                // panic!("No está implementado el caso de la cámara dentro de una esfera");
                // Normalmente se retornaría t2
            }
        }
    }

    fn get_normal_vec(&self, intersection: Vec3) -> Vec3 {
        (intersection - self.center) / self.r
    }

    fn get_texture_coords(&self, intersection: Vec3) -> TextureCoords {
        let spherical_vec = intersection - self.center;
        TextureCoords {
            x: (1.0 + (spherical_vec.z.atan2(spherical_vec.x) as f64) / PI) * 0.5,
            y: (spherical_vec.y / self.r).acos() as f64 / PI,
        }
    }

    fn get_params(&self) -> &ObjectParameters {
        &self.params
    }
}

pub struct TextureCoords {
    pub x: f64,
    pub y: f64,
}

fn checker_pattern<T: ShapeCalculations>(coords: TextureCoords, object: &T) -> Color {
    type Int = i32;
    let int_x = (((coords.x / object.checkerboard()).floor()) % Int::MAX as f64) as Int;
    let int_y = (((coords.y / object.checkerboard()).floor()) % Int::MAX as f64) as Int;

    if (int_x + int_y) % 2 == 0 {
        object.color()
    } else {
        colors::BLACK
    }
}

#[derive(Clone, Debug)]
pub struct ObjectParameters {
    pub color: Color,
    pub k_a: f64,
    pub k_d: f64,
    pub k_n: f64,
    pub k_s: f64,
    pub o1: f64,
    pub reflection: f64,
    pub transparency: f64,
    pub checkerboard: f64,
}

#[enum_dispatch]
pub trait ShapeCalculations: Sized {
    /// Returns the distance "t" from the camera to the point
    fn get_intersection(&self, ray: &Ray) -> Option<f64>;

    fn get_normal_vec(&self, intersection: Vec3) -> Vec3;
    fn get_texture_coords(&self, intersection: Vec3) -> TextureCoords;

    // This method exists so that all the other parameter getters can have default impls and each
    // struct must only define this method
    fn get_params(&self) -> &ObjectParameters;

    fn get_color_at(&self, point: Vec3) -> Color {
        if self.get_params().checkerboard > 0.0 {
            checker_pattern(self.get_texture_coords(point), self)
        } else {
            self.color()
        }
    }

    fn color(&self) -> Color {
        self.get_params().color
    }

    fn k_a(&self) -> f64 {
        self.get_params().k_a
    }
    fn k_d(&self) -> f64 {
        self.get_params().k_d
    }
    fn k_n(&self) -> f64 {
        self.get_params().k_n
    }
    fn k_s(&self) -> f64 {
        self.get_params().k_s
    }
    fn o1(&self) -> f64 {
        self.get_params().o1
    }
    fn reflection(&self) -> f64 {
        self.get_params().reflection
    }
    fn transparency(&self) -> f64 {
        self.get_params().transparency
    }
    fn checkerboard(&self) -> f64 {
        self.get_params().checkerboard
    }
}

#[enum_dispatch(ShapeCalculations)]
#[derive(Clone, Debug)]
pub enum Shape {
    Sphere,
    Plane,
}
