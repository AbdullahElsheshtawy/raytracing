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
use hittable_list::{HittableList, HittableObject};
use material::Material;
use sphere::Sphere;
use util::{rand, rand_f32};
use vec3::{random_range, random_vec, Vec3};

fn main() {
    let mut world = HittableList::default();

    let ground_material = Material::Lambartian {
        albedo: Vec3::new(0.5, 0.5, 0.5),
    };
    world.add(HittableObject::Sphere(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand_f32();
            let center = Vec3::new(
                a as f32 + 0.9 * rand_f32(),
                0.2,
                b as f32 + 0.9 * rand_f32(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                match choose_mat {
                    0.0..0.8 => {
                        // Diffuse
                        let albedo = random_vec() * random_vec();
                        let sphere_material = Material::Lambartian { albedo };
                        world.add(HittableObject::Sphere(Sphere::new(
                            center,
                            0.2,
                            sphere_material,
                        )));
                    }

                    0.8..0.95 => {
                        // Metal
                        let albedo = random_range(0.5, 1.0);
                        let fuzz = rand(0.0, 0.5);
                        let sphere_material = Material::Metal { albedo, fuzz };
                        world.add(HittableObject::Sphere(Sphere::new(
                            center,
                            0.2,
                            sphere_material,
                        )));
                    }

                    _ => {
                        // Glass
                        let sphere_material = Material::Dialetric {
                            refraction_index: 1.5,
                        };
                        world.add(HittableObject::Sphere(Sphere::new(
                            center,
                            0.2,
                            sphere_material,
                        )));
                    }
                }
            }
        }
    }

    let material1 = Material::Dialetric {
        refraction_index: 1.5,
    };
    world.add(HittableObject::Sphere(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Material::Lambartian {
        albedo: Vec3::new(0.4, 0.2, 0.1),
    };

    world.add(HittableObject::Sphere(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Material::Metal {
        albedo: Vec3::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    };

    world.add(HittableObject::Sphere(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 10;
    cam.max_depth = 10;

    cam.vfov = 20.0;
    cam.look_from = Vec3::new(13.0, 2.0, 3.0);
    cam.look_at = Vec3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;

    cam.render(&world);
}
