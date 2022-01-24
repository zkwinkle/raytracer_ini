use anyhow::Result;
use itertools::multiunzip;
use sdl_wrapper::ScreenContextManager;
use std::path::Path;

use crate::constants::{MAX_REFLECTIONS, SHADOWS, TOLERANCE, TOLERANCE_MUL};
use crate::scene::{Light, Observer, Scene};
use crate::shapes::{Color, Ray, Shape, ShapeCalculations};
use crate::vec3::Vec3;

pub fn raytrace<P: AsRef<Path>>(
    path: P,
    observer: &Observer,
    scene: &Scene,
    screen: &mut ScreenContextManager,
) -> Result<()> {
    let ratio_x = (observer.max_p.x - observer.min_p.x) / f64::from(screen.get_width());
    let ratio_y = (observer.max_p.y - observer.min_p.y) / f64::from(screen.get_height());

    let z_t = observer.plane_z;

    let height = screen.get_height();
    let update_interval = screen.get_width() / 10;

    for i in 0..screen.get_width() {
        for j in 0..screen.get_height() {
            // Get ray
            let x_t = (f64::from(i) + 0.5) * ratio_x + observer.min_p.x;
            let y_t = (f64::from(j) + 0.5) * ratio_y + observer.min_p.y;
            let target = Vec3::new(x_t, y_t, z_t);
            let ray = Ray::from_2_points(observer.camera, target);

            // Get color
            let color = get_color_pixel(ray, scene, 1.0, MAX_REFLECTIONS);

            // Paint
            screen.set_color(color.r as f32, color.g as f32, color.b as f32);
            screen.plot_pixel(i, (height - 1) - j); // flip images so they're not upside down
        }
        if i % update_interval == 0 {
            screen.present()?;
        }
    }

    screen.present()?;

    screen.save_img(path)?;

    Ok(())
}

/// o1 = percentage of color that belongs to the current call (relevant for reflections)
fn get_color_pixel(ray: Ray, scene: &Scene, total_o1: f64, reflections: u32) -> Color {
    if let Some(inter) = get_first_intersection(&ray, scene) {
        let normal = inter.object.get_normal_vec(inter.point);
        // bump mapping experiments ( wip / trippy weird stuff, idk how to go about this)
        //let normal = Vec3 {
        //    x: tex.y % (normal.y),
        //    y: tex.x % (normal.x),
        //    z: normal.z % (normal.x),
        //};
        //let normal = (normal + inter.point).normalize().normalize();

        let backwards_vec = -1.0 * ray.dir;

        // Calculate stuff relating to each specific light that has to be reused, for optimization
        // purposes
        let (shadow_intersections, light_factors, l_vecs): (Vec<f64>, Vec<f64>, Vec<Vec3>) =
            multiunzip(scene.get_lights().iter().map(|light| {
                (
                    if SHADOWS {
                        get_shadow_intersection(
                            &Ray::from_2_points(inter.point, light.position).advance(TOLERANCE),
                            scene,
                            light,
                        )
                    } else {
                        0.0
                    },
                    // F_att * Ip
                    light.get_attenuation((light.position - inter.point).norm()) * light.intensity,
                    // L vectors
                    light.get_l_vec(inter.point),
                )
            }));

        let total_intensity = (scene
            .get_lights()
            .iter()
            .enumerate()
            .map(|(i, light)| {
                let mut intensity: f64 =
                    (l_vecs[i].dot(normal)).max(0.0) * light_factors[i] * inter.object.k_d();

                if SHADOWS {
                    intensity *= shadow_intersections[i]
                }

                light.color * intensity
            })
            .sum::<Color>()
            + (scene.ambient_color * scene.ambient * inter.object.k_a()))
        .min(1.0);

        let rgb_d = total_intensity * inter.object.get_color_at(inter.point);

        let total_speculation = (scene
            .get_lights()
            .iter()
            .enumerate()
            .map(|(i, light)| {
                let reflection_vec: Vec3 = 2.0 * normal * (normal.dot(l_vecs[i])) - l_vecs[i];

                let mut specular: f64 = (reflection_vec.dot(backwards_vec))
                    .max(0.0)
                    .powf(inter.object.k_n())
                    * light_factors[i]
                    * inter.object.k_s();

                if SHADOWS {
                    specular *= shadow_intersections[i];
                }

                (light.color - rgb_d) * specular
            })
            .sum::<Color>())
        .min(1.0);

        let object_color = rgb_d + total_speculation;

        let o1 = inter.object.o1();
        if o1 < 1.0 && total_o1 > TOLERANCE * TOLERANCE_MUL {
            let transparency_c = if inter.object.transparency() > TOLERANCE {
                let refraction_dir = get_refractive_dir(&ray);

                // We advance the anchor a bit (a TOLERANCE amount) to avoid the sphere getting stuck
                let transparency_vec = Ray {
                    anchor: inter.point,
                    dir: refraction_dir,
                }
                .advance(TOLERANCE);

                get_color_pixel(
                    transparency_vec,
                    scene,
                    total_o1 * inter.object.transparency(),
                    reflections,
                )
            } else {
                object_color
            };

            let reflection_c = if inter.object.reflection() > TOLERANCE && reflections > 0 {
                let reflection_dir = ray.dir - 2.0 * (ray.dir.dot(normal)) * normal;

                // We advance the anchor a bit (a TOLERANCE amount) to avoid the sphere getting stuck
                // reflecting itself due to float rounding error
                let reflection_vec = Ray {
                    anchor: inter.point,
                    dir: reflection_dir,
                }
                .advance(TOLERANCE);

                get_color_pixel(
                    reflection_vec,
                    scene,
                    total_o1 * inter.object.reflection(),
                    reflections - 1,
                )
            } else {
                object_color
            };
            o1 * (object_color)
                + inter.object.reflection() * reflection_c
                + inter.object.transparency() * transparency_c
        } else {
            object_color
        }
    } else {
        scene.bg_color
    }
}

struct Intersection<'a> {
    //t: f64,
    object: &'a Shape,
    point: Vec3,
}

fn get_first_intersection<'a>(ray: &Ray, scene: &'a Scene) -> Option<Intersection<'a>> {
    // Init tmin and the intersected shape
    let mut tmin = f64::INFINITY;
    let mut intersection: Option<Intersection> = None;

    for object in scene.get_objects() {
        if let Some(t) = object.get_intersection(ray) {
            if t < tmin {
                tmin = t;
                intersection = Some(Intersection {
                    //t: tmin,
                    object,
                    point: ray.point_at_t(tmin),
                });
            }
        }
    }

    intersection
}

/// Returns the total transparency of the intersection, if there's no intersection then it reports
/// 1.0 (total transparency)
fn get_shadow_intersection<'a>(ray: &Ray, scene: &'a Scene, light: &Light) -> f64 {
    let t_light: f64 = (light.position - ray.anchor).norm();

    for object in scene.get_objects() {
        if let Some(t) = object.get_intersection(ray) {
            if t < t_light && t > TOLERANCE {
                // revisamos t > TOLERANCE para que el objeto no se auto-detecte como intersección
                let total_transparency = if object.transparency() > 0.0 {
                    object.transparency()
                        * get_shadow_intersection(
                            &Ray {
                                anchor: ray.point_at_t(t),
                                dir: get_refractive_dir(ray),
                            }
                            .advance(TOLERANCE),
                            scene,
                            light,
                        )
                } else {
                    0.0
                };
                return total_transparency;
            }
        }
    }

    1.0
}

fn get_refractive_dir(ray: &Ray) -> Vec3 {
    // Since we're doing non-refractive transparency this doesn't change anything,  keeping it here
    // to add refraction in the future

    ray.dir
}
