extern crate cgmath;

use cgmath::Vector3;
use cgmath::InnerSpace;

use crate::rendering::scene::Scene;

// pub struct ImagePlane {
// }

pub struct Ray {
    pub origin:    Vector3<f64>,
    pub direction: Vector3<f64>
}

impl Ray {

    /// Creating a ray with an origin and a direction.
    pub fn new(origin: Vector3<f64>, direction: Vector3<f64>) -> Ray {
        Ray {
            origin,
            direction
        }
    }

    /// Creating an empty ray at zero coordinates.
    pub fn zero() -> Ray {
        Ray {
            origin:    Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            direction: Vector3 { x: 0.0, y: 0.0, z: 0.0 }
        }
    }

    /// The image plane will look like this:
    ///
    ///    1
    /// -1 *---*---* 1
    ///    |   |   |
    ///    *---*---*
    ///    |   |   |
    ///    *---*---*
    ///   -1
    ///
    /// We need to transform pixels into the scene's coordinates, that's why we use an "image plane".
    pub fn create_prime(x: u32, y: u32, scene: &Scene) -> Ray {

        // We first have to look for the aspect ratio of the image.
        let aspect_ratio = if scene.width > scene.height {
            scene.width as f64 / scene.height as f64
        } else {
            1.0
        };

        // We stil need the fov slide, but I don't know how to do it yet.
        // let fov = ??;

        // Calculating x and y coordinates as the [-1; 1] plan coordinates.
        // Mul by 2 and sub by 1 transforms the result in the [-1; 1] bounds.
        // For the y plane coordinates, where calculating the inverse because
        // positive y are going downwards.
        let image_plane_x =  (((x as f64) / scene.width  as f64) * 2.0 - 1.0) * aspect_ratio;
        let image_plane_y = -((y as f64) / scene.height as f64) * 2.0 + 1.0;

        Ray {
            origin:    Vector3::new(0.0, 0.0, 0.0),
            // Normalizing the vector ([1, 1, 1] unit vector).
            // z: -1 is because the plane faces the camera and is -1.0 unit in front of it.
            direction: Vector3::new(image_plane_x, image_plane_y, -1.0).normalize()
        }
    }
}
