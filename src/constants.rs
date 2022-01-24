/// Tolerance with wich floating point comparisons are carried out
pub const TOLERANCE: f64 = 0.00001;

/// Default color of scene's background
pub const DEFAULT_BG_COLOR: &str = "#3D1A28";
pub const DEFAULT_LIGHT_COLOR: &str = "#FFFFFF";

/// Default values for parameters
pub const DEFAULT_HARDNESS: f64 = 10.0;

/// flag for calculating shadows
pub const SHADOWS: bool = true;

/// max number of recursive calls due to reflection
pub const MAX_REFLECTIONS: u32 = 5;

/// Default values for args
pub const DEFAULT_RES: u32 = 600;
pub const DEFAULT_SCENE: &str = "config/basic_scene.ini";
pub const DEFAULT_OBSERVER: &str = "config/basic_observer.ini";
pub const DEFAULT_IMAGE: &str = "images/out.png";
