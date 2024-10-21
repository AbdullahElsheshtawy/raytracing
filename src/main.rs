mod camera;
mod hittable;
mod hittable_list;
mod interval;
mod ray;
mod sphere;
mod util;
mod vec3;
use camera::Camera;
use hittable_list::HittableList;
use sphere::Sphere;
use vec3::Vec3;

fn write_color(color: &Vec3) {
    let (r, g, b) = (color.x(), color.y(), color.z());

    // Translate all the [0, 1] component values to the byte range [0, 255].
    let rb = (255.99 * r) as u8;
    let gb = (255.99 * g) as u8;
    let bb = (255.99 * b) as u8;

    println!("{} {} {}", rb, gb, bb);
}

fn main() {
    let mut world = HittableList::default();

    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -2.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let mut cam = Camera::new(400, 16.0 / 9.0);
    cam.render(&world);
}
