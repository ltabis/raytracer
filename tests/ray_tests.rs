use raytracer::props::ray::Ray;

#[test]
fn instantiate_ray_test() {

    let ray_1: Ray = Ray::new(10.0, 2.5, 5.5);

    assert_eq!(ray_1.origin.x, 10.0);
    assert_eq!(ray_1.origin.y, 2.5);
    assert_eq!(ray_1.origin.z, 5.5);

    assert_eq!(ray_1.direction.x, 0.0);
    assert_eq!(ray_1.direction.y, 0.0);
    assert_eq!(ray_1.direction.z, 0.0);

    let ray_2: Ray = Ray::zero();

    assert_eq!(ray_2.origin.x, 0.0);
    assert_eq!(ray_2.origin.y, 0.0);
    assert_eq!(ray_2.origin.z, 0.0);

    assert_eq!(ray_2.direction.x, 0.0);
    assert_eq!(ray_2.direction.y, 0.0);
    assert_eq!(ray_2.direction.z, 0.0);
}
