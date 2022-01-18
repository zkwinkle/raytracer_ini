use anyhow::{anyhow, Error, Result};
use configparser::ini::Ini;
use std::path::Path;

use crate::shapes::{Color, Shape, ShapeCalculations, Sphere};
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

    pub fn prepare(mut self, camera: Vec3) -> Self {
        for shape in &mut self.objects {
            shape.prepare(camera);
        }
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

impl Observer {
    pub fn read_config<P: AsRef<Path>>(path: P) -> Result<Observer> {
        let mut config = Ini::new();

        let map = config.load(path).map_err(|s| anyhow!(s))?;

        println!("{:?}", map);

        let camera = Vec3 {
            x: get_float_fails(&config, "camera", "x")?,
            y: get_float_fails(&config, "camera", "y")?,
            z: get_float_fails(&config, "camera", "z")?,
        };

        let min_p = Vec3 {
            x: get_float_fails(&config, "plane", "x_min")?,
            y: get_float_fails(&config, "plane", "y_min")?,
            z: get_float_default(&config, "plane", "z", 0.0)?,
        };

        let max_p = Vec3 {
            x: get_float_fails(&config, "plane", "x_max")?,
            y: get_float_fails(&config, "plane", "y_max")?,
            z: get_float_default(&config, "plane", "z", 0.0)?,
        };

        Ok(Observer {
            camera,
            min_p,
            max_p,
        })
    }
}

fn get_float_default(config: &Ini, section: &str, key: &str, default: f64) -> Result<f64> {
    Ok(config
        .getfloat(section, key)
        .map_err(|s| anyhow!(s))?
        .unwrap_or(default))
}

fn get_float_fails(config: &Ini, section: &str, key: &str) -> Result<f64> {
    Ok(config
        .getfloat(section, key)
        .map_err(|s| anyhow!(s))?
        .ok_or_else(|| {
            anyhow!(
                "Missing attribute '{}' for {} in observer config file",
                key,
                section
            )
        })?)
}
