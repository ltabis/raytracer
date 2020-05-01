extern crate cgmath;
use cgmath::Vector3;

use raytracer::props::{material::{Texture, Material}, color::Color};
use raytracer::rendering;

use image;
use image::GenericImageView;

#[test]
fn test_can_render_scene() {

    let scene = rendering::scene::Scene {
        width: 10,
        height: 10,
        fov: 90.0,
        objects: vec![
            Box::new(rendering::sphere::Sphere {
                center: Vector3::new(0.0, 0.0, -3.0),
                radius: 1.0,
                material: Material {
                    texture: Texture::ColorTexture(Color {
                        r: 1.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    }),
                    albedo: 0.35,
                }
            }
            ),
            Box::new(rendering::sphere::Sphere {
                center: Vector3::new(-0.5, 0.0, -3.0),
                radius: 1.0,
                material: Material {
                    texture: Texture::ColorTexture(Color {
                        r: 0.0,
                        g: 0.0,
                        b: 1.0,
                        a: 1.0,
                    }),
                    albedo: 0.35,
                }
            }
            ),
            Box::new(rendering::sphere::Sphere {
                center: Vector3::new(0.5, 0.0, -3.0),
                radius: 1.0,
                material: Material {
                    texture: Texture::ColorTexture(Color {
                        r: 0.0,
                        g: 1.0,
                        b: 0.0,
                        a: 1.0,
                    }),
                    albedo: 0.35,
                }
            }
            ),
            Box::new(rendering::plane::Plane {
                origin: Vector3::new(0.0, -3.0, -5.0),
                normal: Vector3::new(0.0, -1.0, 0.0),
                material: Material {
                    texture: Texture::ColorTexture(Color {
                        r: 1.0,
                        g: 1.0,
                        b: 1.0,
                        a: 1.0,
                    }),
                    albedo: 0.35,
                }
            }
            ),
            Box::new(rendering::plane::Plane {
                origin: Vector3::new(0.0, 0.0, -5.0),
                normal: Vector3::new(0.0, 0.0, -1.0),
                material: Material {
                    texture: Texture::ColorTexture(Color {
                        r: 0.5,
                        g: 0.5,
                        b: 0.5,
                        a: 1.0,
                    }),
                    albedo: 0.35,
                }
            }
            )
        ],
        lights: vec![],
    };

    let img: image::DynamicImage = match rendering::renderer::render(&scene) {
        Some(image) => image,
        None => panic!("There is no object to render.")
    };
    assert_eq!(scene.width, img.width());
    assert_eq!(scene.height, img.height());
}
