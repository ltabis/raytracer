extern crate cgmath;
use cgmath::{ Vector2, Vector3, InnerSpace };

use crate::props::{ ray::Ray, material::Material };
use crate::rendering::object_traits::Drawable;

/// A sphere struct that contains data to render a sphere in a scene.
pub struct Plane {
    pub origin:   Vector3<f64>,
    pub normal:   Vector3<f64>,
    pub material: Material
}

impl Drawable for Plane {

    // A function to calculate the distance between a point on the plane and the origin
    // of the ray. The maths are from scratchpixel.
    fn hit(&self, ray: &Ray) -> Option<f64> {

        // We start by calculating the scalar product between the normal vector
        // of the plane and the ray vector. In our equation, il will divide some
        // other stuff, so we need to check if the value is equal to zero.
        let dot_product = self.normal.dot(ray.direction);

        // If the product is equal to zero, then the ray is parallel to the plane,
        // there is not intersection.
        if dot_product <= 1e-6 {
            return None;
        }

        // maths explained at scratchpixel.
        // link : ray-plane-and-ray-disk-intersection.
        let distance = (self.origin - ray.origin).dot(self.normal) / dot_product;

        if distance >= 0.0 {
            return Some(distance);
        }
        None
    }

    fn surface_normal(&self, _hit: Vector3<f64>) -> Vector3<f64> {
        -self.normal
    }

    fn material_data(&self) -> &Material {
        &self.material
    }

    fn get_texture_coords(&self, _: Vector3<f64>) -> Vector2<f64> {
        Vector2 {
            x: 0.0,
            y: 0.0
        }
    }
}
