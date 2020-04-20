use cgmath::Vector3;
use raytracer::rendering::plane::Plane;
use raytracer::rendering::scene::Scene;
use raytracer::props::color::Color;
use raytracer::props::ray::Ray;

#[test]
fn intersect_plane_test() {

    let scene = Scene {
        width: 1000,
        height: 1000,
        fov: 90.0,
        objects: vec![
            Box::new(Plane {
                origin: Vector3::new(0.0, -3.0, -5.0),
                normal: Vector3::new(0.0, -1.0,  0.0),
                color:  Color {
                    r: 255.0,
                    g: 255.0,
                    b: 255.0,
                    a: 255.0,
                }
            }
            )
        ]
    };

    let ray = Ray::create_prime(900, 900, &scene);

    match scene.objects[0].hit(&ray) {
        Some(_) => assert!(true),
        None     => assert!(false)
    };
}

#[test]
fn do_not_intersect_plane_test() {

    let scene = Scene {
        width: 1000,
        height: 1000,
        fov: 90.0,
        objects: vec![
            Box::new(Plane {
                origin: Vector3::new(0.0, -3.0, -5.0),
                normal: Vector3::new(0.0, -1.0,  0.0),
                color:  Color {
                    r: 255,
                    g: 255,
                    b: 255,
                    a: 255,
                }
            }
            )
        ]
    };

    let ray = Ray::create_prime(100, 200, &scene);

    match scene.objects[0].hit(&ray) {
        Some(_) => assert!(false),
        None    => assert!(true)
    };
}

use raytracer::rendering::object_traits::Drawable;

#[test]
fn get_plane_color_test() {

    let plane_1 = Plane {
        origin: Vector3::new(0.0, 0.0, -5.0),
        normal: Vector3::new(1.0, 1.0, 1.0),
        color:  Color {
            r: 128,
            g: 255,
            b: 50,
            a: 10,
        }
    };

    let plane_ref = plane_1.color();

    assert_eq!(plane_1.color.r, plane_ref.r);
    assert_eq!(plane_1.color.g, plane_ref.g);
    assert_eq!(plane_1.color.b, plane_ref.b);
    assert_eq!(plane_1.color.a, plane_ref.a);
}
