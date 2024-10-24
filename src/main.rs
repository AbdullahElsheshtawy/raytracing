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

    let material_left = Material::Dialetric {
        refraction_index: 1.50,
    };
    let material_bubble = Material::Dialetric {
        refraction_index: 1.00 / 1.50,
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
        Vec3::new(-1.0, 0.0, -1.0),
        0.4,
        material_bubble,
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.vfov = 20.0;
    cam.look_from = Vec3::new(-2.0, 2.0, 1.0);
    cam.look_at = Vec3::new(0.0, 0.0, -1.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 10.0;
    cam.focus_dist = 3.4;

    cam.render(&world);
}
