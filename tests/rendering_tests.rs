extern crate cgmath;
use cgmath::Vector3;

use raytracer::props;
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
                color: props::color::Color {
                    r: 255,
                    g: 0,
                    b: 0,
                    a: 255
                }
            }
            ),
            Box::new(rendering::sphere::Sphere {
                center: Vector3::new(-0.5, 0.0, -3.0),
                radius: 1.0,
                color: props::color::Color {
                    r: 0,
                    g: 0,
                    b: 255,
                    a: 255
                }
            }
            ),
            Box::new(rendering::sphere::Sphere {
                center: Vector3::new(0.5, 0.0, -3.0),
                radius: 1.0,
                color: props::color::Color {
                    r: 0,
                    g: 255,
                    b: 0,
                    a: 255
                }
            }
            ),
            Box::new(rendering::plane::Plane {
                origin: Vector3::new(0.0, -3.0, -5.0),
                normal: Vector3::new(0.0, -1.0, 0.0),
                color: props::color::Color {
                    r: 255,
                    g: 255,
                    b: 255,
                    a: 50
                }
            }
            ),
            Box::new(rendering::plane::Plane {
                origin: Vector3::new(0.0, 0.0, -5.0),
                normal: Vector3::new(0.0, 0.0, -1.0),
                color: props::color::Color {
                    r: 128,
                    g: 128,
                    b: 128,
                    a: 50
                }
            }
            )
        ]
    };

    let img: image::DynamicImage = match rendering::renderer::render(&scene) {
        Some(image) => image,
        None => panic!("There is no object to render.")
    };
    assert_eq!(scene.width, img.width());
    assert_eq!(scene.height, img.height());
}
