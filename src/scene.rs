use anyhow::{anyhow, Result};
use configparser::ini::Ini;
use std::path::Path;

use crate::constants::{DEFAULT_BG_COLOR, DEFAULT_HARDNESS, DEFAULT_LIGHT_COLOR};
use crate::shapes::{Color, Shape, Sphere};
use crate::vec3::Vec3;

pub struct Scene {
    objects: Vec<Shape>,
    lights: Vec<Light>,
    pub ambient: f64,
    pub bg_color: Color,
    pub ambient_color: Color,
}

impl Scene {
    pub fn get_objects(&self) -> &Vec<Shape> {
        &self.objects
    }
    pub fn get_lights(&self) -> &Vec<Light> {
        &self.lights
    }

    pub fn read_config<P: AsRef<Path>>(path: P) -> Result<Scene> {
        let mut config = Ini::new();
        let mut objects = Vec::<Shape>::new();
        let mut lights = Vec::<Light>::new();

        config.set_comment_symbols(&[';', '"']);
        config.load(path).map_err(|s| anyhow!(s))?;
        //println!("Map: {:?}", map);

        let ambient = get_float_fails(&config, "scene", "I_a")?;
        let bg_color = get_color_default(&config, "scene", "bg_color", DEFAULT_BG_COLOR)?;
        let ambient_color =
            get_color_default(&config, "scene", "ambient_color", DEFAULT_LIGHT_COLOR)?;

        // spheres (checks for prefix)
        for sphere_section in config
            .sections()
            .iter()
            .filter(|s| s.len() >= 6 && &s[0..6] == "sphere")
        {
            let center_x = get_float_fails(&config, sphere_section, "center_x")?;
            let center_y = get_float_fails(&config, sphere_section, "center_y")?;
            let center_z = get_float_fails(&config, sphere_section, "center_z")?;
            let center = Vec3::new(center_x, center_y, center_z);

            let radius = get_float_fails(&config, sphere_section, "radius")
                .or_else(|_| get_float_fails(&config, sphere_section, "r"))?;

            let color = get_color_fails(&config, sphere_section)?;
            let k_d = get_float_fails(&config, sphere_section, "k_d")?;
            let k_a = get_float_default(&config, sphere_section, "k_a", 1.0)?;
            let k_s = get_float_fails(&config, sphere_section, "k_s")?;
            let k_n = get_float_default(&config, sphere_section, "k_n", DEFAULT_HARDNESS)?;

            objects.push(Shape::Sphere(Sphere::new(
                center, radius, color, k_a, k_d, k_n, k_s,
            )));
        }

        // lights
        for light_section in config
            .sections()
            .iter()
            .filter(|s| s.len() >= 5 && &s[0..5] == "light")
        {
            let x = get_float_fails(&config, light_section, "x")?;
            let y = get_float_fails(&config, light_section, "y")?;
            let z = get_float_fails(&config, light_section, "z")?;
            let position = Vec3::new(x, y, z);

            let intensity = get_float_fails(&config, light_section, "intensity")
                .or_else(|_| get_float_fails(&config, light_section, "I_p"))?;

            let c_1 = get_float_fails(&config, light_section, "c_1")
                .or_else(|_| get_float_fails(&config, light_section, "C1"))?;
            let c_2 = get_float_fails(&config, light_section, "c_2")
                .or_else(|_| get_float_fails(&config, light_section, "C2"))?;
            let c_3 = get_float_fails(&config, light_section, "c_3")
                .or_else(|_| get_float_fails(&config, light_section, "C3"))?;

            let color = get_color_default(&config, light_section, "color", DEFAULT_LIGHT_COLOR)?;

            lights.push(Light {
                position,
                intensity,
                c_1,
                c_2,
                c_3,
                color,
            })
        }

        Ok(Scene {
            objects,
            lights,
            ambient,
            bg_color,
            ambient_color,
        })
    }
}

pub struct Light {
    pub position: Vec3,
    pub intensity: f64,
    c_1: f64,
    c_2: f64,
    c_3: f64,
    pub color: Color,
}

impl Light {
    pub fn get_attenuation(&self, distance: f64) -> f64 {
        (1.0_f64 / (self.c_1 + self.c_2 * distance + self.c_3 * distance * distance)).min(1.0)
    }

    pub fn get_l_vec(&self, intersection: Vec3) -> Vec3 {
        (self.position - intersection).normalize()
    }
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

        let plane_z = get_float_default(&config, "plane", "z", 0.0)?;

        let min_p = Vec3 {
            x: get_float_fails(&config, "plane", "x_min")?,
            y: get_float_fails(&config, "plane", "y_min")?,
            z: plane_z,
        };

        let max_p = Vec3 {
            x: get_float_fails(&config, "plane", "x_max")?,
            y: get_float_fails(&config, "plane", "y_max")?,
            z: plane_z,
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
        .ok_or_else(|| anyhow!("Missing attribute '{}' for {} in config file", key, section))?)
}

fn get_color_fails(config: &Ini, section: &str) -> Result<Color> {
    Color::from_hex(&config.get(section, "color").ok_or_else(|| {
        anyhow!(
            "Missing color attribute in section '{}' in config file",
            section
        )
    })?)
}

fn get_color_default(config: &Ini, section: &str, key: &str, default: &str) -> Result<Color> {
    Color::from_hex(config.get(section, key).as_deref().unwrap_or(default))
}
