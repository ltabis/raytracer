use raytracer::rendering::plane::Plane;
use raytracer::rendering::scene::Scene;
use raytracer::props::color::Color;
use raytracer::props::{material::{Texture, Material}, ray::Ray};

#[test]
fn intersect_plane_test() {

    let scene = Scene {
        width: 1000,
        height: 1000,
        fov: 90.0,
        objects: vec![
            Box::new(Plane {
                origin: (0.0, -3.0, -5.0).into(),
                normal: (0.0, -1.0,  0.0).into(),
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
            )
        ],
        lights: vec![],
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
                origin: (0.0, -3.0, -5.0).into(),
                normal: (0.0, -1.0,  0.0).into(),
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
            )
        ],
        lights: vec![],
    };

    let ray = Ray::create_prime(100, 200, &scene);

    match scene.objects[0].hit(&ray) {
        Some(_) => assert!(false),
        None    => assert!(true)
    };
}

#[test]
fn get_plane_color_test() {

    let plane_1 = Plane {
        origin: (0.0, 0.0, -5.0).into(),
        normal: (1.0, 1.0, 1.0).into(),
        material: Material {
            texture: Texture::ColorTexture(Color {
                r: 0.5,
                g: 1.0,
                b: 0.25,
                a: 1.0,
            }),
            albedo: 0.35,
        }
    };

    let plane_ref = plane_1.material.texture.color((0.0, 0.0).into());

    assert_eq!(plane_1.material.texture.color((0.0, 0.0).into()).r, plane_ref.r);
    assert_eq!(plane_1.material.texture.color((0.0, 0.0).into()).g, plane_ref.g);
    assert_eq!(plane_1.material.texture.color((0.0, 0.0).into()).b, plane_ref.b);
    assert_eq!(plane_1.material.texture.color((0.0, 0.0).into()).a, plane_ref.a);
}
