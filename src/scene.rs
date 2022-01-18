use crate::shapes::{Color, Shape, Sphere};
use crate::vec3::Vec3;

pub struct Scene {
    objects: Vec<Shape>,
    lights: Vec<Light>,
}

impl Scene {
    pub fn get_objects(&self) -> &Vec<Shape> {
        &self.objects
    }
    pub fn get_lights(&self) -> &Vec<Light> {
        &self.lights
    }
    pub fn new() -> Scene {
        Scene {
            objects: Vec::new(),
            lights: Vec::new(),
        }
    }
    pub fn add_sphere(mut self, center: Vec3, radius: f64, color: Color) -> Self {
        self.objects
            .push(Shape::Sphere(Sphere::new(center, radius, color)));
        self
    }
}

pub struct Light {
    pub position: Vec3,
    pub intensity: f64,
}

pub struct Observer {
    pub camera: Vec3,

    /// minimum point of the projection plane
    pub min_p: Vec3,
    /// maximum point of the projection plane
    pub max_p: Vec3,
}
