use anyhow::{Error, Result};
use image::{ImageBuffer, Rgb, RgbImage};
use std::path::Path;

/// This struct abstracts away any direct interaction with the SDL module, so that the user may
/// only need to call the provided methods without `use`ing any sdl modules.
pub struct ScreenContextManager {
    framebuffer: RgbImage,
    color: Rgb<u8>,
    height: u32,
    width: u32,
}

impl ScreenContextManager {
    /// Creates a new object, with the side-effect of creating a new window with the title given.
    pub fn new(width: u32, height: u32) -> ScreenContextManager {
        ScreenContextManager {
            // Create empty framebuffer
            framebuffer: ImageBuffer::new(width, height),
            color: Rgb([0, 0, 0]),
            height,
            width,
        }
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }
    pub fn get_height(&self) -> u32 {
        self.height
    }

    /// Sets the color to be used for drawing operations.
    /// Parameters correspond to RGB colors and must be real numbers in the range [0, 1].
    pub fn set_color(&mut self, r: f32, g: f32, b: f32) {
        self.color = Rgb([
            (r * 255.0).round() as u8,
            (g * 255.0).round() as u8,
            (b * 255.0).round() as u8,
        ]);
    }

    /// Plots a single pixel on the framebuffer.
    pub fn plot_pixel(&mut self, x: u32, y: u32) {
        self.framebuffer.put_pixel(x, y, self.color);
    }

    #[allow(dead_code)]
    /// Clears the entire framebuffer with a grey shadow given by a real number in the range [0,
    /// 1].
    pub fn clear(&mut self, shadow: f32) {
        let shadow = (shadow * 255.0).round() as u8;
        self.framebuffer.fill(shadow);
    }

    #[allow(dead_code)]
    /// Clears the entire framebuffer with the given color.
    /// Parameters correspond to RGB colors and must be real numbers in the range [0, 1].
    pub fn clear_with_rgb(&mut self, r: f32, g: f32, b: f32) {
        let color = Rgb([
            (r * 255.0).round() as u8,
            (g * 255.0).round() as u8,
            (b * 255.0).round() as u8,
        ]);

        self.framebuffer = ImageBuffer::from_pixel(self.width, self.height, color);
    }

    /// Saves the current framebuffer as an image whose format is derived from the file extension.
    pub fn save_img<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let buffer = bytemuck::cast_slice(&self.framebuffer);
        image::save_buffer(
            path,
            buffer,
            self.width,
            self.height,
            image::ColorType::Rgb8,
        )
        .map_err(Error::msg)
    }
}
