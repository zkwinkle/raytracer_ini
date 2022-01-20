use anyhow::Result;
use sdl_wrapper::ScreenContextManager;
use std::path::Path;

use crate::constants::{BACKGROUND_COLOR, SHADOWS, TOLERANCE};
use crate::scene::{Light, Observer, Scene};
use crate::shapes::{Color, Shape, ShapeCalculations};
use crate::vec3::Vec3;

#[derive(Debug)]
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
}

impl Ray {
    pub fn point_at_t(&self, t: f64) -> Vec3 {
        self.anchor + self.dir * t
    }
}

pub fn raytrace<P: AsRef<Path>>(
    path: P,
    observer: &Observer,
    scene: &Scene,
    screen: &mut ScreenContextManager,
) -> Result<()> {
    let ratio_x = (observer.max_p.x - observer.min_p.x) / f64::from(screen.get_width());
    let ratio_y = (observer.max_p.y - observer.min_p.y) / f64::from(screen.get_height());

    let z_t = observer.min_p.z;

    let height = screen.get_height();

    for i in 0..screen.get_width() {
        for j in 0..screen.get_height() {
            // Get ray
            let x_t = (f64::from(i) + 0.5) * ratio_x + observer.min_p.x;
            let y_t = (f64::from(j) + 0.5) * ratio_y + observer.min_p.y;
            let target = Vec3::new(x_t, y_t, z_t);
            let ray = Ray::from_2_points(observer.camera, target);

            // Get color
            let color = get_color_pixel(ray, scene);

            // Paint
            screen.set_color(color.r, color.g, color.b);
            screen.plot_pixel(i, (height - 1) - j); // flip images so they're not upside down
            screen.present()?;
        }
    }

    screen.save_img(path)?;

    Ok(())
}

fn get_color_pixel(ray: Ray, scene: &Scene) -> Color {
    if let Some(inter) = get_first_intersection(&ray, scene) {
        let normal = inter.object.get_normal_vec(inter.point);

        let intensity = (scene
            .get_lights()
            .iter()
            .map(|light| {
                if !SHADOWS
                    || get_shadow_intersection(
                        &Ray::from_2_points(inter.point, light.position),
                        scene,
                        light,
                    )
                    .is_none()
                {
                    (light.get_l_vec(inter.point).dot(normal)).max(0.0)
                        * light.intensity
                        * light.get_attenuation((light.position - inter.point).norm())
                } else {
                    0.0
                }
            })
            .sum::<f64>()
            * inter.object.k_d()
            + (scene.ambient * inter.object.k_a()))
        .min(1.0);

        intensity as f32 * inter.object.get_color()
    } else {
        BACKGROUND_COLOR
    }
}

struct Intersection<'a> {
    t: f64,
    object: &'a Shape,
    point: Vec3,
}

fn get_first_intersection<'a>(ray: &Ray, scene: &'a Scene) -> Option<Intersection<'a>> {
    // Init tmin and the intersected shape
    let mut tmin = f64::INFINITY;
    let mut intersection: Option<Intersection> = None;

    for object in scene.get_objects() {
        if let Some(t) = object.get_intersection(&ray) {
            if t < tmin {
                tmin = t;
                intersection = Some(Intersection {
                    t: tmin,
                    object: &object,
                    point: ray.point_at_t(tmin),
                });
            }
        }
    }

    intersection
}

fn get_shadow_intersection<'a>(
    ray: &Ray,
    scene: &'a Scene,
    light: &Light,
) -> Option<Intersection<'a>> {
    let t_light: f64 = (light.position - ray.anchor).norm();

    let mut intersection: Option<Intersection> = None;

    for object in scene.get_objects() {
        if let Some(t) = object.get_intersection(&ray) {
            println!("Object for which shadow happened: {:?}", object);
            println!("Shadow for ray: {:?}\nat t: {}\n", ray, t);
            if t < t_light && t > TOLERANCE {
                // revisamos t > TOLERANCE para que el objeto no se auto-detecte como intersección
                intersection = Some(Intersection {
                    t,
                    object: &object,
                    point: ray.point_at_t(t),
                });
                return intersection;
            }
        }
    }

    intersection
}
