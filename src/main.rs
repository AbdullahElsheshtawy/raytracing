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
use interval::Interval;
use sphere::Sphere;
use vec3::Vec3;

fn write_color(color: &Vec3) {
    let (r, g, b) = (color.x(), color.y(), color.z());

    // Translate all the [0, 1] component values to the byte range [0, 255].
    let intensity: Interval = Interval::new(0.000, 0.999);
    let rb = (256_f32 * intensity.clamp(r)) as i32;
    let gb = (256_f32 * intensity.clamp(g)) as i32;
    let bb = (256_f32 * intensity.clamp(b)) as i32;
    println!("{} {} {}", rb, gb, bb);
}

fn main() {
    let mut world = HittableList::default();

    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -2.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let mut cam = Camera::new(400, 16.0 / 9.0);
    cam.samples_per_pixel = 100;
    cam.render(&world);
}
