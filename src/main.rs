extern crate cgmath;
use cgmath::Vector3;

use raytracer::props;
use raytracer::rendering;

use image;

fn main() {
    let scene = rendering::scene::Scene {
        width: 1920,
        height: 1080,
        fov: 90.0,
        objects: vec![
            Box::new(rendering::sphere::Sphere {
                center: Vector3::new(0.0, 2.0, -3.0),
                radius: 1.0,
                material: props::material::Material {
                    texture: props::material::Texture::ImageTexture(
                        image::open("resources/concrete.jpg").unwrap()
                    ),
                    albedo: 0.38
                }
            }),
            Box::new(rendering::sphere::Sphere {
                center: Vector3::new(-1.0, 0.0, -3.0),
                radius: 1.0,
                material: props::material::Material {
                    texture: props::material::Texture::ColorTexture(
                        props::color::Color {
                            r: 1.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0
                        }
                    ),
                    albedo: 0.18
                }
            }),
            Box::new(rendering::sphere::Sphere {
                center: Vector3::new(1.0, 0.0, -3.0),
                radius: 1.0,
                material: props::material::Material {
                    texture: props::material::Texture::ColorTexture(
                        props::color::Color {
                            r: 0.2,
                            g: 0.5,
                            b: 0.3,
                            a: 1.0
                        }
                    ),
                    albedo: 0.18
                }
            }),
            Box::new(rendering::plane::Plane {
                origin: Vector3::new(0.0, -2.0, 0.0),
                normal: Vector3::new(0.0, -1.0, 0.0),
                material: props::material::Material {
                    texture: props::material::Texture::ColorTexture(
                        props::color::Color {
                            r: 1.0,
                            g: 1.0,
                            b: 1.0,
                            a: 0.2
                            }),
                    albedo: 0.18
                }
            }),
            Box::new(rendering::plane::Plane {
                origin: Vector3::new(0.0, 0.0, -20.0),
                normal: Vector3::new(0.0, 0.0, -1.0),
                material: props::material::Material {
                    texture: props::material::Texture::ColorTexture(
                        props::color::Color {
                            r: 0.5,
                            g: 0.5,
                            b: 0.5,
                            a: 0.2
                        }),
                    albedo: 0.18
                }
            }),
        ],
        lights: vec![
            Box::new(rendering::light::SphericalLight {
                origin: Vector3::new(0.0, 0.0, -1.0),
                color: props::color::Color {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    a: 1.0
                },
                intensity: 500.0
            }),
            Box::new(rendering::light::DirectionalLight {
                direction: Vector3::new(0.25, -0.5, -0.5),
                color: props::color::Color {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    a: 1.0
                },
                intensity: 5.0
            })
        ]
    };

    let img: image::DynamicImage = match rendering::renderer::render(&scene) {
        Some(image) => image,
        None => panic!("There is no object to render."),
    };
    img.save("test.png").unwrap();
}
