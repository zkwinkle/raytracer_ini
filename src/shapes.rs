use anyhow::{anyhow, Result};
use enum_dispatch::enum_dispatch;
use std::iter::Sum;
use std::ops;

use crate::raytracer::Ray;
use crate::vec3::Vec3;

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

impl ops::Mul<Color> for Color {
    type Output = Color;
    fn mul(self, other: Color) -> Self::Output {
        Color {
            r: self.r.mul(other.r),
            g: self.g.mul(other.g),
            b: self.b.mul(other.b),
        }
    }
}

impl ops::Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            r: (self.r.add(other.r)).min(1.0),
            g: (self.g.add(other.g)).min(1.0),
            b: (self.b.add(other.b)).min(1.0),
        }
    }
}

impl Sum<Self> for Color {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(colors::BLACK, |acc, c| Self {
            r: acc.r + c.r,
            g: acc.g + c.g,
            b: acc.b + c.b,
        })
    }
}

fn is_hex_format(hex: &str) -> bool {
    hex.starts_with('#') && hex.len() == 7 && hex[1..].chars().all(|d| d.is_digit(16))
}

#[derive(Copy, Clone, Debug)]
pub struct Sphere {
    center: Vec3,
    r: f64,
    color: Color,
    k_a: f64,
    k_d: f64,
}

impl Sphere {
    pub fn new(center: Vec3, r: f64, color: Color, k_a: f64, k_d: f64) -> Sphere {
        Sphere {
            center,
            r,
            color,
            k_a,
            k_d,
        }
    }
}

impl ShapeCalculations for Sphere {
    /// Retorn el valor "t" que se usaría en el rayo para llegar la intersección
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

    fn get_color(&self) -> Color {
        self.color
    }

    fn k_a(&self) -> f64 {
        self.k_a
    }
    fn k_d(&self) -> f64 {
        self.k_d
    }
}

#[enum_dispatch]
pub trait ShapeCalculations {
    /// Returns the distance "t" from the camera to the point
    fn get_intersection(&self, ray: &Ray) -> Option<f64>;

    fn get_normal_vec(&self, point: Vec3) -> Vec3;

    fn get_color(&self) -> Color;

    fn k_a(&self) -> f64;
    fn k_d(&self) -> f64;
}

#[enum_dispatch(ShapeCalculations)]
#[derive(Clone, Debug)]
pub enum Shape {
    Sphere,
}
