extern crate cgmath;
use cgmath::{ Vector2, Vector3 };

use crate::props::{ ray::Ray, color::Color, material::Material };

pub trait Drawable {

    // Ray computation.
    fn hit(&self, ray: &Ray) -> Option<f64>;
    fn surface_normal(&self, hit: Vector3<f64>) -> Vector3<f64>;

    // Gets all material data;
    fn material_data(&self) -> &Material;

    // Gets the coordinates of the texture at a given hit point.
    fn get_texture_coords(&self, hit: Vector3<f64>) -> Vector2<f64>;
}

pub trait Light {

    fn direction_from(&self, hit :Vector3<f64>) -> Vector3<f64>;
    fn intensity(&self, hit :Vector3<f64>) -> f64;
    fn distance(&self, hit :Vector3<f64>) -> f64;
    fn color(&self) -> &Color;
}