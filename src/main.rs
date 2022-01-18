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
    // scene stuff
    let scene = Scene::read_config("config/basic_scene.ini")?;
    //let scene = scene.add_sphere(Vec3::new(20.0, 10.0, 15.0), 3.0, Color::new(0.1, 0.8, 0.3)?);

    let observer = Observer::read_config("config/basic_observer.ini")?;

    let prepped_scene = scene.prepare(observer.camera);

    // sdl screen
    let mut screen = ScreenContextManager::new("Ray Tracing Challenge", 150, 150)?;

    // raytrace :)
    raytrace(
        "images/tortilla.png",
        &observer,
        &prepped_scene,
        &mut screen,
    )?;

    Ok(())
}
