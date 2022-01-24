use anyhow::{anyhow, Result};
use enum_dispatch::enum_dispatch;
use std::iter::Sum;
use std::ops;

use crate::constants::TOLERANCE;
use crate::vec3::Vec3;

#[derive(Debug, Clone)]
pub struct Ray {
    pub anchor: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn from_2_points(p_origin: Vec3, p_target: Vec3) -> Ray {
        // println!("Ray from origin: {:?}\tto target: {:?}", p_origin, p_target);
        // println!("target - og: {:?}", p_target - p_origin);
        let v_dir: Vec3 = (p_target - p_origin).normalize();
        Ray {
            anchor: p_origin,
            dir: v_dir,
        }
    }

    pub fn advance(self, t: f64) -> Ray {
        Ray {
            anchor: self.anchor + t * self.dir,
            dir: self.dir,
        }
    }

    pub fn point_at_t(&self, t: f64) -> Vec3 {
        self.anchor + self.dir * t
    }
}

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
            x: plane_vec.dot(x_axis),
            y: plane_vec.dot(y_axis),
        }
    }

    fn get_params(&self) -> &ObjectParameters {
        &self.params
    }
}

#[derive(Clone, Debug)]
pub struct Disc {
    normal: Vec3,
    center: Vec3,
    r: f64,
    params: ObjectParameters,
}

impl Disc {
    pub fn new(normal: Vec3, center: Vec3, r: f64, params: ObjectParameters) -> Disc {
        Disc {
            center,
            normal: normal.normalize(),
            r,
            params,
        }
    }
}

impl ShapeCalculations for Disc {
    /// Returns the distance "t" from the camera to the point
    fn get_intersection(&self, ray: &Ray) -> Option<f64> {
        let normal = self.normal;
        let denominator = normal.dot(ray.dir);

        if denominator.abs() < TOLERANCE {
            None
        } else {
            let t = 1.0 * (self.center - ray.anchor).dot(normal) / denominator;
            // Check it's in front of camera + inside radius
            if t > 0.0 && (ray.point_at_t(t) - self.center).norm() <= self.r {
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

        let plane_vec = intersection - self.center;

        TextureCoords {
            x: plane_vec.dot(x_axis),
            y: plane_vec.dot(y_axis),
        }
    }

    fn get_params(&self) -> &ObjectParameters {
        &self.params
    }
}

#[derive(Clone, Debug)]
pub struct Triangle {
    normal: Vec3,
    a: Vec3,
    b: Vec3,
    c: Vec3,
    params: ObjectParameters,
}

impl Triangle {
    pub fn new(a: Vec3, b: Vec3, c: Vec3, params: ObjectParameters) -> Triangle {
        let normal = (b - a).cross(c - a).normalize();
        Triangle {
            a,
            b,
            c,
            normal,
            params,
        }
    }
}

impl ShapeCalculations for Triangle {
    /// Returns the distance "t" from the camera to the point
    fn get_intersection(&self, ray: &Ray) -> Option<f64> {
        let normal = self.normal;
        let denominator = normal.dot(ray.dir);

        if denominator.abs() < TOLERANCE {
            None
        } else {
            let t = 1.0 * (self.a - ray.anchor).dot(normal) / denominator;

            // get barycentric coords
            // ref: https://math.stackexchange.com/a/544947
            let p = ray.point_at_t(t);

            let u = self.b - self.a;
            let v = self.c - self.a;

            let n = u.cross(v);
            let w = p - self.a;

            let n2 = n.dot(n);
            let gamma = (u.cross(w).dot(n)) / n2;
            let beta = (w.cross(v).dot(n)) / n2;
            let alpha = 1.0 - gamma - beta;

            // Check it's in front of camera
            if t > 0.0
                && 0.0 <= alpha
                && alpha <= 1.0
                && 0.0 <= beta
                && beta <= 1.0
                && 0.0 <= gamma
                && gamma <= 1.0
            {
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

        let plane_vec = intersection - self.a;

        TextureCoords {
            x: plane_vec.dot(x_axis),
            y: plane_vec.dot(y_axis),
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
        //let circumference = 2.0 * PI * self.r;
        TextureCoords {
            x: 2.0 * self.r * (1.0 + (spherical_vec.z.atan2(spherical_vec.x) as f64)),
            y: 2.0 * self.r * (spherical_vec.y / self.r).acos() as f64,
        }
    }

    fn get_params(&self) -> &ObjectParameters {
        &self.params
    }
}

#[derive(Clone, Debug)]
pub struct Cylinder {
    ray: Ray,
    r: f64,
    length: f64,
    params: ObjectParameters,
}

impl Cylinder {
    pub fn new(anchor: Vec3, dir: Vec3, r: f64, length: f64, params: ObjectParameters) -> Cylinder {
        Cylinder {
            ray: Ray {
                anchor,
                dir: dir.normalize(),
            },
            r,
            length,
            params,
        }
    }

    fn get_length_at_inter(&self, intersection: Vec3) -> f64 {
        let l = intersection - self.ray.anchor;
        l.dot(self.ray.dir)
    }
}

impl ShapeCalculations for Cylinder {
    /// Returns the distance "t" from the camera to the point
    fn get_intersection(&self, ray: &Ray) -> Option<f64> {
        //// First we displace the ray's anchor to align with the origin
        let displaced_anchor =
            ray.anchor
                .translation(-self.ray.anchor.x, -self.ray.anchor.y, -self.ray.anchor.z);

        // then, figure out rotation matrix
        let rotation = self.ray.dir.to_align(Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        });

        // then rotate
        let rotated_dir = ray.dir.apply_matrix(rotation);
        let rotated_anchor = displaced_anchor.apply_matrix(rotation);

        let fixed_ray = Ray {
            anchor: rotated_anchor,
            dir: rotated_dir,
        };

        // Now we find the intersection with the cylinder whose base is at the origin and direction
        // is aligned with 'y'

        let dir = fixed_ray.dir;
        let anchor = fixed_ray.anchor;

        let a = dir.x * dir.x + dir.z * dir.z;
        let b = 2.0 * (dir.x * anchor.x + dir.z * anchor.z);
        let c = anchor.x * anchor.x + anchor.z * anchor.z - self.r * self.r;

        let determinant = (b * b - 4.0 * a * c).sqrt();

        if determinant.is_nan() {
            None
        } else {
            let t1 = (-b - determinant) / (2.0 * a);
            let t2 = (-b + determinant) / (2.0 * a);

            let t1_d = self.get_length_at_inter(ray.point_at_t(t1));
            let t2_d = self.get_length_at_inter(ray.point_at_t(t2));

            if t1 > 0.0 {
                if t1_d <= self.length && t1_d > 0.0 {
                    Some(t1)
                } else if t2_d <= self.length && t2_d > 0.0 {
                    Some(t2)
                } else {
                    None
                }
            } else if t2 < 0.0 {
                None
            } else if t2_d <= self.length && t2_d > 0.0 {
                Some(t2)
            } else {
                None
            }
        }
    }

    fn get_normal_vec(&self, intersection: Vec3) -> Vec3 {
        //println!("Normal intersection at: {:?}", intersection);
        let d = self.get_length_at_inter(intersection);
        let v_m = self.ray.point_at_t(d);

        (intersection - v_m).normalize()
    }

    fn get_texture_coords(&self, intersection: Vec3) -> TextureCoords {
        //// First we displace the ray's anchor to align with the origin
        let displaced_intersection =
            intersection.translation(-self.ray.anchor.x, -self.ray.anchor.y, -self.ray.anchor.z);

        // then, figure out rotation matrix
        let rotation = self.ray.dir.to_align(Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        });

        // then rotate
        let rotated_intersection = displaced_intersection.apply_matrix(rotation);

        TextureCoords {
            x: self.r * (1.0 + rotated_intersection.z.atan2(rotated_intersection.x) as f64),
            y: rotated_intersection.y,
        }
    }

    fn get_params(&self) -> &ObjectParameters {
        &self.params
    }
}

#[derive(Clone, Debug)]
pub struct Cone {
    ray: Ray,
    length: f64,
    params: ObjectParameters,
    slope: f64,
}

impl Cone {
    pub fn new(
        anchor: Vec3,
        dir: Vec3,
        length: f64,
        k1: f64,
        k2: f64,
        params: ObjectParameters,
    ) -> Cone {
        Cone {
            ray: Ray {
                anchor,
                dir: dir.normalize(),
            },
            length,
            params,
            slope: k2 / k1,
        }
    }

    fn get_length_at_inter(&self, intersection: Vec3) -> f64 {
        let l = intersection - self.ray.anchor;
        l.dot(self.ray.dir)
    }

    fn r_at(&self, length: f64) -> f64 {
        length * self.slope
    }
}

impl ShapeCalculations for Cone {
    /// Returns the distance "t" from the camera to the point
    fn get_intersection(&self, ray: &Ray) -> Option<f64> {
        //// First we displace the ray's anchor to align with the origin
        let displaced_anchor =
            ray.anchor
                .translation(-self.ray.anchor.x, -self.ray.anchor.y, -self.ray.anchor.z);

        // then, figure out rotation matrix
        let rotation = self.ray.dir.to_align(Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        });

        // then rotate
        let rotated_dir = ray.dir.apply_matrix(rotation);
        let rotated_anchor = displaced_anchor.apply_matrix(rotation);

        let fixed_ray = Ray {
            anchor: rotated_anchor,
            dir: rotated_dir,
        };

        // Now we find the intersection with the cylinder whose base is at the origin and direction
        // is aligned with 'y'

        let dir = fixed_ray.dir;
        let anchor = fixed_ray.anchor;

        let slope2 = self.slope * self.slope;
        let a = dir.x * dir.x + dir.z * dir.z - dir.y * dir.y * slope2;
        let b = 2.0 * (dir.x * anchor.x + dir.z * anchor.z - dir.y * anchor.y * slope2);
        let c = anchor.x * anchor.x + anchor.z * anchor.z - anchor.y * anchor.y * slope2;

        let determinant = (b * b - 4.0 * a * c).sqrt();

        if determinant.is_nan() {
            None
        } else {
            let t1 = (-b - determinant) / (2.0 * a);
            let t2 = (-b + determinant) / (2.0 * a);

            let t1_d = self.get_length_at_inter(ray.point_at_t(t1));
            let t2_d = self.get_length_at_inter(ray.point_at_t(t2));

            if t1 > 0.0 {
                if t1_d <= self.length && t1_d > 0.0 {
                    Some(t1)
                } else if t2_d <= self.length && t2_d > 0.0 {
                    Some(t2)
                } else {
                    None
                }
            } else if t2 < 0.0 {
                None
            } else if t2_d <= self.length && t2_d > 0.0 {
                Some(t2)
            } else {
                None
            }
        }
    }

    fn get_normal_vec(&self, intersection: Vec3) -> Vec3 {
        //println!("Normal intersection at: {:?}", intersection);
        let d = self.get_length_at_inter(intersection);
        let v_m = self.ray.point_at_t(d);

        (intersection - v_m).normalize()
    }

    fn get_texture_coords(&self, intersection: Vec3) -> TextureCoords {
        //// First we displace the ray's anchor to align with the origin
        let displaced_intersection =
            intersection.translation(-self.ray.anchor.x, -self.ray.anchor.y, -self.ray.anchor.z);

        // then, figure out rotation matrix
        let rotation = self.ray.dir.to_align(Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        });

        // then rotate
        let rotated_intersection = displaced_intersection.apply_matrix(rotation);

        TextureCoords {
            x: self.r_at(self.get_length_at_inter(intersection))
                * (1.0 + rotated_intersection.z.atan2(rotated_intersection.x) as f64),
            y: rotated_intersection.y,
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
    Cylinder,
    Cone,
    Plane,
    Disc,
    Triangle,
}
