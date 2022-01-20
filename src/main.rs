mod constants;
mod raytracer;
mod scene;
mod shapes;
mod vec3;

use anyhow::Result;
use sdl_wrapper::ScreenContextManager;

use raytracer::raytrace;
use scene::{Observer, Scene};

fn main() -> Result<()> {
    // scene stuff
    let scene = Scene::read_config("config/basic_scene.ini")?;

    let observer = Observer::read_config("config/basic_observer.ini")?;

    // sdl screen
    let mut screen = ScreenContextManager::new("Ray Tracing Challenge", 150, 150)?;

    // raytrace :)
    raytrace("images/shadows.png", &observer, &scene, &mut screen)?;

    Ok(())
}
