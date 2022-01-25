mod constants;
mod raytracer;
mod scene;
mod screen;
mod shapes;
mod vec3;

use anyhow::Result;
use screen::ScreenContextManager;

use clap::Parser;
use constants::{DEFAULT_IMAGE, DEFAULT_OBSERVER, DEFAULT_RES, DEFAULT_SCENE};
use raytracer::raytrace;
use scene::{Observer, Scene};
use std::{thread::sleep, time::Duration};

fn main() -> Result<()> {
    // Parse args
    let args = Args::parse();

    // scene stuff
    let scene = Scene::read_config(args.scene)?;

    let observer = Observer::read_config(args.observer)?;

    // sdl screen
    let mut screen = ScreenContextManager::new(args.resolution, args.resolution);

    // raytrace :)
    raytrace(args.image, &observer, &scene, &mut screen)?;

    sleep(Duration::from_millis(900));

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
    #[clap(short='O', long, default_value = DEFAULT_OBSERVER)]
    observer: String,

    /// Path to image output (image format is determined by file extension)
    #[clap(short='o', long, default_value = DEFAULT_IMAGE)]
    image: String,
}
