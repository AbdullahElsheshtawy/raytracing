mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod sphere;
mod util;
mod vec3;
use camera::Camera;
use hittable_list::HittableList;
use material::Material;
use sphere::Sphere;
use vec3::Vec3;

fn main() {
    let mut world = HittableList::default();
    let material_ground = Material::Lambartian {
        albedo: Vec3::new(0.8, 0.8, 0.0),
    };

    let material_center = Material::Lambartian {
        albedo: Vec3::new(0.1, 0.2, 0.5),
    };

    let material_left = Material::Metal {
        albedo: Vec3::new(0.8, 0.8, 0.8),
        fuzz: 0.3,
    };
    let material_right = Material::Metal {
        albedo: Vec3::new(0.8, 0.6, 0.2),
        fuzz: 1.0,
    };

    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));

    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));
    let mut cam = Camera::new(1920, 16.0 / 9.0);
    cam.samples_per_pixel = 1000;
    cam.max_depth = 50;
    cam.render(&world);
}
