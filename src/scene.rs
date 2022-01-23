use anyhow::{anyhow, Context, Error, Result};
use configparser::ini::Ini;
use std::path::Path;

use crate::constants::{DEFAULT_BG_COLOR, DEFAULT_HARDNESS, DEFAULT_LIGHT_COLOR};
use crate::shapes::{Color, ObjectParameters, Plane, Shape, Sphere};
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
            let center = get_vec3_fails(&config, sphere_section, "center")?;

            let radius = get_float_fails(&config, sphere_section, "radius")
                .or_else(|_| get_float_fails(&config, sphere_section, "r"))?;

            let params = get_params(&config, sphere_section)?;

            objects.push(Shape::Sphere(Sphere::new(center, radius, params)));
        }

        for plane_section in config
            .sections()
            .iter()
            .filter(|s| s.len() >= 5 && &s[0..5] == "plane")
        {
            let point = get_vec3_fails(&config, plane_section, "point")?;

            let normal = get_vec3_fails(&config, plane_section, "normal")?;

            let params = get_params(&config, plane_section)?;

            objects.push(Shape::Plane(Plane::new(normal, point, params)));
        }

        // lights
        for light_section in config
            .sections()
            .iter()
            .filter(|s| s.len() >= 5 && &s[0..5] == "light")
        {
            let position = get_vec3_fails(&config, light_section, "position")?;

            let intensity = get_float_fails(&config, light_section, "intensity")
                .or_else(|_| get_float_fails(&config, light_section, "I_p"))?
                .max(0.0);

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

        let camera = get_vec3_fails(&config, "camera", "position")?;

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
    config
        .getfloat(section, key)
        .map_err(|s| anyhow!(s))?
        .ok_or_else(|| anyhow!("Missing attribute '{}' for {} in config file", key, section))
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

fn get_vec3_fails(config: &Ini, section: &str, key: &str) -> Result<Vec3> {
    let mut vec_string = config.get(section, key).ok_or_else(|| {
        anyhow!(
            "Missing vector attribute '{}' in section {} of config file",
            key,
            section
        )
    })?;

    let first_char: char = vec_string.trim().chars().next().unwrap();

    let valid_delimiters: Option<[&str; 2]> = match first_char {
        '[' => Some(["[", "]"]),
        '(' => Some(["(", ")"]),
        '0'..='9' => None,
        _ => return Err(anyhow!("In vector attribute '{}' in section {} the first element is not a valid delimiter or a valid number: {}", key, section, first_char)),
    };

    if let Some(delimiters) = valid_delimiters {
        vec_string = vec_string
            .trim()
            .strip_prefix(delimiters[0])
            .unwrap()
            .strip_suffix(delimiters[1])
            .ok_or_else(|| {
                anyhow!(
                    "In vector attribute '{}' in section {} the vector isn't terminated by the matching closing delimiter '{}'",
                    key,
                    section,
                    delimiters[1]
                )
            })?.to_string();
    }

    let num_strs = vec_string.split(',');
    let floats: Vec<f64> = num_strs
        .map(|s| s.trim().parse::<f64>().map_err(Error::msg))
        .collect::<Result<Vec<f64>>>().context(format!("In vector attribute '{}' in section {} the vector's elements aren't valid floating point numbers", key, section))?;

    if floats.len() != 3 {
        return Err(anyhow!("In vector attribute '{}' in section {} the vector supplied should be 3-dimensional and it's currently {}-dimensional", key, section, floats.len()));
    }

    Ok(Vec3::new(floats[0], floats[1], floats[2]))
}

fn get_params(config: &Ini, section: &str) -> Result<ObjectParameters> {
    let color = get_color_fails(config, section)?;
    let k_d = get_float_fails(config, section, "k_d")?.clamp(0.0, 1.0);
    let k_a = get_float_default(config, section, "k_a", 1.0)?.clamp(0.0, 1.0);
    let k_s = get_float_fails(config, section, "k_s")?.clamp(0.0, 1.0);
    let k_n = get_float_default(config, section, "k_n", DEFAULT_HARDNESS)?.max(1.0);
    let reflection = get_float_default(config, section, "reflection", 0.0)?.clamp(0.0, 1.0);
    let transparency = get_float_default(config, section, "transparency", 0.0)?.clamp(0.0, 1.0);
    let checkerboard = get_float_default(config, section, "checkerboard", 0.0)?.max(0.0);

    if reflection + transparency > 1.0 {
        return Err(anyhow!("In section '{}' the transparency+reflection > 1. The transparecy + reflection must not sum to more than 1, please lower the values.", section));
    }

    let o1 = 1.0 - (reflection + transparency);

    Ok(ObjectParameters {
        color,
        k_d,
        k_a,
        k_s,
        k_n,
        o1,
        reflection,
        transparency,
        checkerboard,
    })
}
