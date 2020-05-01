use cgmath::Vector3;
use raytracer::rendering::sphere::Sphere;
use raytracer::rendering::scene::Scene;
use raytracer::props::{material::{Texture, Material}, color::Color, ray::Ray};



#[test]
fn intersect_sphere_test() {

    let scene = Scene {
        width: 1000,
        height: 1000,
        fov: 90.0,
        objects: vec![
            Box::new(Sphere {
                center: Vector3::new(0.0, 0.0, -5.0),
                radius: 2.0,
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

    let ray = Ray::create_prime(500, 500, &scene);

    match scene.objects[0].hit(&ray) {
        Some(_) => assert!(true),
        None    => assert!(false)
    };
}

#[test]
fn do_not_intersect_sphere_test() {

    let scene = Scene {
        width: 1000,
        height: 1000,
        fov: 90.0,
        objects: vec![
            Box::new(Sphere {
                center: Vector3::new(0.0, 0.0, -5.0),
                radius: 1.0,
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

    let ray = Ray::create_prime(100, 100, &scene);

    match scene.objects[0].hit(&ray) {
        Some(_) => assert!(false),
        None    => assert!(true)
    };
}

#[test]
fn get_sphere_color_test() {

    let sphere_1 = Sphere {
        center: Vector3::new(0.0, 0.0, -5.0),
        radius: 1.0,
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

    let plane_ref = sphere_1.material.texture.color((0.0, 0.0).into());

    assert_eq!(sphere_1.material.texture.color((0.0, 0.0).into()).r, plane_ref.r);
    assert_eq!(sphere_1.material.texture.color((0.0, 0.0).into()).g, plane_ref.g);
    assert_eq!(sphere_1.material.texture.color((0.0, 0.0).into()).b, plane_ref.b);
    assert_eq!(sphere_1.material.texture.color((0.0, 0.0).into()).a, plane_ref.a);
}
