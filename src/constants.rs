use crate::shapes::Color;

/// Tolerance with wich floating point comparisons are carried out
const TOLERANCE: f64 = 0.001;

/// Color of scene's background
pub const BACKGROUND_COLOR: Color = Color {
    r: 0.239215686,
    g: 0.1019607843,
    b: 0.1568627451,
};