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
    scene = scene.add_sphere(
        Vec3::new(10.0, 10.0, 10.0),
        10.0,
        Color::new(0.1, 0.3, 0.8)?,
    );
    //scene = scene.add_sphere(Vec3::new(20.0, 10.0, 15.0), 3.0, Color::new(0.1, 0.8, 0.3)?);

    let observer = Observer::read_config("config/basic_observer.ini")?;

    let prepped_scene = scene.prepare(observer.camera);
    // sdl screen
    let mut screen = ScreenContextManager::new("Ray Tracing Challenge", 150, 150)?;

    raytrace(
        "images/tortilla.png",
        &observer,
        &prepped_scene,
        &mut screen,
    )?;

    Ok(())
}
