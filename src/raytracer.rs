use anyhow::Result;
use sdl_wrapper::ScreenContextManager;
use std::path::Path;

use crate::constants::BACKGROUND_COLOR;
use crate::scene::{Observer, Scene};
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
    if let Some(obj) = get_first_intersection(ray, scene) {
        obj.get_color()
    } else {
        BACKGROUND_COLOR
    }
}

fn get_first_intersection(ray: Ray, scene: &Scene) -> Option<&Shape> {
    // Init tmin and the intersected shape
    let mut tmin = f64::INFINITY;
    let mut shape: Option<&Shape> = None;

    //println!("Finding intersection for ray:\n{:?}", ray);

    for object in scene.get_objects() {
        if let Some(t) = object.get_intersection(&ray) {
            if t < tmin {
                tmin = t;
                shape = Some(&object)
            }
        }
    }

    shape
}
