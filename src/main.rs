mod constants;
mod raytracer;
mod scene;
mod shapes;
mod vec3;

use anyhow::Result;
use sdl_wrapper::ScreenContextManager;

use raytracer::raytrace;
use scene::{Observer, Scene};
use shapes::Color;
use vec3::Vec3;

fn main() -> Result<()> {
    // vec 3 stuff
    let v = Vec3::new(10.5, 69.5, 30.5);
    let w = Vec3::new(5.3, 69.5, -10.0);
    println!(
        "norm: {}\n normalized: {:?}\n v-w: {:?},\n w-v: {:?}\n w+v: {:?}\n normalized norm: {}",
        v.norm(),
        v.normalize(),
        v - w,
        w - v,
        v + w,
        w.normalize().norm()
    );

    let vnorm = v.normalize();
    let wnorm = w.normalize();

    println!(
        "\n normalized \"norm\" without sqrt:\n\tv: {}\n\tw: {}",
        vnorm.x.powi(2) + vnorm.y.powi(2) + vnorm.z.powi(2),
        wnorm.x.powi(2) + wnorm.y.powi(2) + wnorm.z.powi(2)
    );

    // create scene with one sphere
    let mut scene = Scene::new();
    scene = scene.add_sphere(Vec3::new(10.0, 10.0, 10.0), 5.0, Color::new(0.1, 0.3, 0.8)?);

    let observer = Observer {
        camera: Vec3::new(10.0, 10.0, -5.0),
        min_p: Vec3::new(0.0, 0.0, 0.0),
        max_p: Vec3::new(20.0, 20.0, 0.0),
    };

    // sdl screen
    let mut screen = ScreenContextManager::new("Ray Tracing Challenge", 150, 150)?;

    raytrace(&observer, &scene, &mut screen)?;

    Ok(())
}
