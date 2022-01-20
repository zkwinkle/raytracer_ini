mod constants;
mod raytracer;
mod scene;
mod shapes;
mod vec3;

use anyhow::Result;
use sdl_wrapper::ScreenContextManager;

use clap::Parser;
use constants::{DEFAULT_OBSERVER, DEFAULT_RES, DEFAULT_SCENE};
use raytracer::raytrace;
use scene::{Observer, Scene};

fn main() -> Result<()> {
    // Parse args
    let args = Args::parse();

    // scene stuff
    let scene = Scene::read_config(args.scene)?;

    let observer = Observer::read_config(args.observer)?;

    // sdl screen
    let mut screen =
        ScreenContextManager::new("Ray Tracing Challenge", args.resolution, args.resolution)?;

    // raytrace :)
    raytrace("images/specular.png", &observer, &scene, &mut screen)?;

    Ok(())
}

/// Raytracer that reads .ini config files and generates an image.
#[derive(Parser, Debug)]
#[clap(author, about, long_about = None)]
struct Args {
    #[clap(default_value_t = DEFAULT_RES)]
    resolution: u32,

    /// Path to scene's config file
    #[clap(short, long, default_value = DEFAULT_SCENE)]
    scene: String,

    /// Path to observer's config file
    #[clap(short, long, default_value = DEFAULT_OBSERVER)]
    observer: String,
}
