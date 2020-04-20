extern crate cgmath;
use cgmath::Vector3;
use cgmath::InnerSpace;

use crate::props::color::Color;
use crate::rendering::object_traits::Light;

pub struct DirectionalLight {

    pub direction: Vector3<f64>,
    pub color:     Color,
    pub intensity: f64
}

impl Light for DirectionalLight {

    fn direction_from(&self, _ :Vector3<f64>) -> Vector3<f64> {
        (-self.direction).normalize()
    }

    fn intensity(&self, _ :Vector3<f64>) -> f64 {
        self.intensity
    }

    fn distance(&self, _ :Vector3<f64>) -> f64 {
        std::f64::INFINITY
    }

    fn color(&self) -> &Color {
        &self.color
    }
}

pub struct SphericalLight {

    pub origin:    Vector3<f64>,
    pub color:     Color,
    pub intensity: f64
}

impl SphericalLight {
    
    fn norm(&self, hit :Vector3<f64>) -> f64 {
        let tmp = self.origin - hit;
        tmp.x * tmp.x + tmp.y * tmp.y + tmp.z * tmp.z
    }
}

impl Light for SphericalLight {
    
    fn direction_from(&self, hit :Vector3<f64>) -> Vector3<f64> {
        (self.origin - hit).normalize()
    }

    fn intensity(&self, hit :Vector3<f64>) -> f64 {
        self.intensity / (4.0 * std::f64::consts::PI * self.norm(hit))
    }

    fn distance(&self, hit :Vector3<f64>) -> f64 {
        self.norm(hit).sqrt()
    }

    fn color(&self) -> &Color {
        &self.color
    }
}