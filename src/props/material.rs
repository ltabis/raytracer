extern crate cgmath;
use cgmath::Vector2;

extern crate image;
use image::{ DynamicImage, GenericImageView, Pixel };

use crate::props::color::Color;

pub enum Texture {
    ColorTexture(Color),
    ImageTexture(DynamicImage)
}

// Converts texture coordinates to window pixel size.
fn wrap(val: f32, bound: u32) -> u32 {

    let signed_bound = bound as i32;
    let float_coord = val * bound as f32;
    let wrapped_coord = (float_coord as i32) % signed_bound;

    if wrapped_coord < 0 {
        (wrapped_coord + signed_bound) as u32
    } else {
        wrapped_coord as u32
    }
}

impl Texture {

    pub fn color(&self, texture_coords: Vector2<f64>) -> Color {

        match *self {
            Texture::ColorTexture(ref color)   => Color::clone(color),
            Texture::ImageTexture(ref texture) => {

                let tex_x = wrap(texture_coords.x as f32, texture.width());
                let tex_y = wrap(texture_coords.y as f32, texture.height());
                let pixel_color = texture.get_pixel(tex_x, tex_y).to_rgba();
            
                Color {
                    r: pixel_color[0] as f64 / 255.0,
                    g: pixel_color[1] as f64 / 255.0,
                    b: pixel_color[2] as f64 / 255.0,
                    a: pixel_color[3] as f64 / 255.0 
                }
            }
        }
    }
}

pub struct Material {

    pub texture: Texture,
    pub albedo:  f64,
}