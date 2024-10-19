mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;
use core::f32;
use std::rc::Rc;

use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use ray::Ray;
use sphere::Sphere;
use vec3::*;

const INIFINTY: f32 = f32::INFINITY;

fn write_color(color: &Vec3) {
    let (r, g, b) = (color.x(), color.y(), color.z());

    // Translate all the [0, 1] component values to the byte range [0, 255].
    let rb = (255.99 * r) as u8;
    let gb = (255.99 * g) as u8;
    let bb = (255.99 * b) as u8;

    println!("{} {} {}", rb, gb, bb);
}

fn hit_sphere(center: Vec3, radius: f32, r: &Ray) -> Option<f32> {
    let oc = center - *r.origin();
    let a = r.direction().length_squared();
    let h = dot(r.direction(), &oc);
    let c = oc.length_squared() - radius * radius;
    let discriminant = h * h - a * c;

    if discriminant < 0.0 {
        return None;
    }

    Some((h - f32::sqrt(discriminant)) / a)
}

fn ray_color(r: &Ray, world: &HittableList) -> Vec3 {
    let mut rec = HitRecord::default();

    if world.hit(r, 0.0, INIFINTY, &mut rec) {
        return (rec.normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5;
    }

    let unit_direction = r.direction().normalize();
    let a = 0.5 * (unit_direction.y() + 1.0);

    (Vec3::new(1.0, 1.0, 1.0) * (1.0 - a)) + (Vec3::new(0.5, 0.7, 1.0) * a)
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    // Calculate the image height and ensure that it's atleast 1
    let image_height = image_width as f32 / aspect_ratio;
    assert!(image_height != 1.0);

    // World
    let mut world = HittableList::default();
    world.add(Rc::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * image_width as f32 / image_height;
    let camera_center = Vec3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel
    let pixel_delta_u = viewport_u / image_width;
    let pixel_delta_v = viewport_v / image_height;

    // Calculate the location of the upper left pixel
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

    let upper_left_pixel_loc = ((pixel_delta_u + pixel_delta_v) * 0.5) + viewport_upper_left;

    // Progress Bar
    let pb = ProgressBar::new(1000);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
        )
        .unwrap(),
    );
    // Render
    println!("P3\n {} {}\n255\n", image_width, image_height);

    for j in (0..image_height as i32).progress_with(pb) {
        for i in 0..image_width {
            let pixel_center = upper_left_pixel_loc + (pixel_delta_u * i) + (pixel_delta_v * j);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&ray, &world);
            write_color(&pixel_color);
        }
    }
}
