use crate::shapes::Color;

/// Tolerance with wich floating point comparisons are carried out
pub const TOLERANCE: f64 = 0.001;

/// Color of scene's background
pub const BACKGROUND_COLOR: Color = Color {
    r: 0.239215686,
    g: 0.1019607843,
    b: 0.1568627451,
};

pub const SHADOWS: bool = true;

/// Default values for args

pub const DEFAULT_RES: u32 = 150;
pub const DEFAULT_SCENE: &str = "config/basic_scene.ini";
pub const DEFAULT_OBSERVER: &str = "config/basic_observer.ini";
