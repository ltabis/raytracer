extern crate cgmath;
use cgmath::InnerSpace;

use crate::rendering;
use crate::props;
use crate::rendering::object_traits::Drawable;
use crate::miscellaneous::progressbar::ProgressBar;

use image;
use image::GenericImage;

/// Takes a scene in parameter and render it to an image with the image crate.
pub fn render(scene: &rendering::scene::Scene) -> Option<image::DynamicImage> {

    // Checking if their is something to render.
    if scene.objects.is_empty() {
        return None;
    }

    // Creating a new image object.
    let mut image = image::DynamicImage::new_rgb8(scene.width, scene.height);

    // Dividing the work with four threads, this needs to be dynamic.
    // TODO : Make the number of thread running customizable.
    render_part_of_image(0, scene.width, &scene, &mut image);

    Some(image)
}

/// Renders a single pixel on an image.
fn render_part_of_image(start: u32,
                        end: u32,
                        scene: &rendering::scene::Scene,
                        image: &mut image::DynamicImage) {

    // Creating a progress bar.
    let mut bar = ProgressBar::new((end * scene.height) as f64, 20);

    // Iterating trough the pixels of the image.
    for x in start..end {
        for y in 0..scene.height {
            render_pixel_on_image(x, y, scene, image);
        }
        bar.inc(scene.height as f64);
    }
}

/// Gets the minimum distance of an object that a ray intercepts.
/// Gives us the object in front of the others, if there is an object that the ray actually hit.
fn get_min_distance<'a>(ray: &props::ray::Ray, objects: &'a Vec<Box<dyn Drawable>>) -> (Option<&'a Box<dyn Drawable>>, Option<f64>) {

    // Calculating the closest distance between the camera and an object that the ray hit.
    let mut obj_to_render: Option<&Box<dyn Drawable>> = None;
    let mut curr_dist_min: Option<f64>                = None;

    for object in objects.iter() {

        match object.hit(&ray) {
            Some(d) => match curr_dist_min {
                Some(cd) => if d < cd {
                    obj_to_render = Some(object);
                    curr_dist_min = Some(d);
                },
                None     => {
                    obj_to_render = Some(object);
                    curr_dist_min = Some(d);
                }
            },
            None    => ()
        }
    };

    (obj_to_render, curr_dist_min)
}

/// Renders an image.
fn render_pixel_on_image(x: u32,
                         y: u32,
                         scene: &rendering::scene::Scene,
                         image: &mut image::DynamicImage) {
    // Tracing a ray.
    let light_ray = props::ray::Ray::create_prime(x, y, scene);

    // Getting the distance beteen the closest object hit by the ray.
    let (obj_to_render, curr_dist_min): (Option<&Box<dyn Drawable>>, Option<f64>) = get_min_distance(&light_ray, &scene.objects);

    // Checking if the ray has hit a sphere.
    // (We will add more shapes in the futur)
    match obj_to_render {
        Some(obj_to_render) => {

            // Computing, from the light sources, what color the current pixel will be.
            let color_to_render = compute_light(&light_ray, obj_to_render, scene, curr_dist_min.unwrap());

            image.put_pixel(x, y, image::Rgba([
                (color_to_render.r * 255.0) as u8,
                (color_to_render.g * 255.0) as u8,
                (color_to_render.b * 255.0) as u8,
                (color_to_render.a * 255.0) as u8
            ]));
        },

        None => image.put_pixel(x, y, image::Rgba([0, 0, 0, 0]))
    }
}

/// Computes the lights and shadows of a scene, returning a color for a given pixel.
fn compute_light(light_ray: &props::ray::Ray,
           object: &Box<dyn Drawable>,
           scene: &rendering::scene::Scene,
           distance: f64) -> props::color::Color {

    let object_material = object.material_data();

    // Light computation.
    let mut pixel_color = props::color::Color::black();
    let light_ray_hit = light_ray.origin + (light_ray.direction * distance);
    let surface_normal = object.surface_normal(light_ray_hit);
    let light_reflected = object_material.albedo / std::f64::consts::PI;

    // Texture of the object.
    let object_texture_coords = object.get_texture_coords(light_ray_hit);
    let texture_color = object_material.texture.color(object_texture_coords);

    // Computing light and shadows for every lights in the scene.
    for light in &scene.lights {

        // Direction to the light normalized.
        let direction_to_light_normal = light.direction_from(light_ray_hit);

        // Shadow computation.
        let shadow_ray = props::ray::Ray::new(light_ray_hit + surface_normal * 1e-4, direction_to_light_normal);
        let (_, dst_to_near_obj) = get_min_distance(&shadow_ray, &scene.objects);

        // Calculating light's intensity if their is no shadow.
        // If their is, display a black pixel.
        // TODO : Replace the black pixel by a backgroud pixel.
        let light_intensity = match dst_to_near_obj {
            Some(d) => if d > light.distance(light_ray_hit) { light.intensity(light_ray_hit) } else { 0.0 },
            None    => light.intensity(light_ray_hit)
        };

        // Computing current's light power.
        let light_power = surface_normal.dot(direction_to_light_normal).max(0.0) * light_intensity;
        let light_color = light.color();

        // Calculating the color of the current pixel.
        // We are clamping the colors to prevent bad shadow rendering.
        pixel_color.r += texture_color.r * light_color.r * light_power * light_reflected;
        pixel_color.g += texture_color.g * light_color.g * light_power * light_reflected;
        pixel_color.b += texture_color.b * light_color.b * light_power * light_reflected;
    }
    pixel_color.clamp()
} 