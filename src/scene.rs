use anyhow::{anyhow, Result};
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

    pub fn read_config<P: AsRef<Path>>(path: P) -> Result<Scene> {
        let mut config = Ini::new();
        let mut scene = Scene::new();

        config.set_comment_symbols(&[';', '"']);
        let map = config.load(path).map_err(|s| anyhow!(s))?;
        println!("Map: {:?}", map);

        // spheres (checks for prefix)
        for sphere_section in config.sections().iter().filter(|s| &s[0..6] == "sphere") {
            println!("Section!: {}", sphere_section);

            let center_x = get_float_fails(&config, sphere_section, "center_x")?;
            let center_y = get_float_fails(&config, sphere_section, "center_y")?;
            let center_z = get_float_fails(&config, sphere_section, "center_z")?;
            let center = Vec3::new(center_x, center_y, center_z);

            let radius = get_float_fails(&config, sphere_section, "radius")
                .or_else(|_| get_float_fails(&config, sphere_section, "r"))?;

            let color = get_color_fails(&config, sphere_section)?;

            scene = scene.add_sphere(center, radius, color);
        }

        Ok(scene)
    }
}

pub struct Light {
    pub position: Vec3,
    pub intensity: f64,
}

/// Represents the camera + the projection plane used for the raytracer.
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

        config.load(path).map_err(|s| anyhow!(s))?;

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

fn get_color_fails(config: &Ini, section: &str) -> Result<Color> {
    Color::from_hex(&config.get(section, "color").ok_or_else(|| {
        anyhow!(
            "Missing color attribute in section '{}' in config file",
            section
        )
    })?)
}
