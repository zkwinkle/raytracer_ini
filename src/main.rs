mod constants;
mod raytracer;
mod scene;
mod screen;
mod shapes;
mod vec3;

use anyhow::{Context, Result};
use screen::ScreenContextManager;

use clap::Parser;
use constants::{DEFAULT_IMAGE, DEFAULT_RES};
use raytracer::raytrace;
use scene::{Observer, Scene};
use std::{thread::sleep, time::Duration};

fn main() -> Result<()> {
    // Parse args
    let args = Args::parse();

    let observer_file = {
        if let Some(file) = args.observer {
            file
        } else {
            args.scene.clone()
        }
    };

    // scene stuff
    let scene = Scene::read_config(args.scene)?;

    let observer = Observer::read_config(observer_file).context("Perhaps you need to specify the path to the observer file you want to read, run with '--help' flag for more info.")?;

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
    #[clap(short, long)]
    scene: String,

    /// Path to observer's config file (defaults to same path as scene)
    #[clap(short = 'O', long)]
    observer: Option<String>,

    /// Path to image output
    #[clap(short='o', long, default_value = DEFAULT_IMAGE)]
    image: String,
}
