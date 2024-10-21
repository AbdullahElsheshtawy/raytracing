use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};

use crate::{
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    interval::Interval,
    ray::Ray,
    util::rand_f32,
    write_color, Vec3,
};

pub struct Camera {
    pub aspect_ratio: f32,
    pub image_width: i32,
    pub samples_per_pixel: i32,

    image_height: i32,          // Rendered image height
    pixel_samples_scale: f32,   // Color scale factor for a sum of pixel samples
    center: Vec3,               // Camera center
    upper_left_pixel_loc: Vec3, // Location of pixel (0, 0)
    pixel_delta_u: Vec3,        // Offset to pixel to the right
    pixel_delta_v: Vec3,        // Offset to pixel below
}

impl Camera {
    pub fn new(image_width: i32, aspect_ratio: f32) -> Self {
        Camera {
            aspect_ratio,
            image_width,
            ..Default::default()
        }
    }
    pub fn render(&mut self, world: &HittableList) {
        self.initialize();
        // Progress Bar
        let pb = ProgressBar::new((self.image_width * self.image_height) as u64);
        pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
        )
        .unwrap(),
    );
        // Render
        println!("P3\n {} {}\n255\n", self.image_width, self.image_height);

        for index in (0..(self.image_width * self.image_height)).progress_with(pb) {
            let j = index / self.image_width; // Calculate the row (height)
            let i = index % self.image_width; // Calculate the column (width)
            let mut pixel_color = Vec3::default();
            for _sample in 0..self.samples_per_pixel {
                let r = self.get_ray(i, j);
                pixel_color += self.ray_color(&r, world);
            }
            write_color(&(pixel_color * self.pixel_samples_scale));
        }
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f32 / self.aspect_ratio) as i32;
        assert!(self.image_height > 1);

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f32;

        self.center = Vec3::new(0.0, 0.0, 0.0);

        // Determine viewport dimensions.
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * ((self.image_width / self.image_height) as f32);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width;
        self.pixel_delta_v = viewport_v / self.image_height;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            self.center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

        self.upper_left_pixel_loc =
            viewport_upper_left + ((self.pixel_delta_u + self.pixel_delta_v) * 0.5)
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        // Construct a camera ray originating from the origin directed at randomly sampled
        // point around the pixel location (i, j).
        let offset = self.sample_square();
        let pixel_sample = self.upper_left_pixel_loc
            + ((i as f32 + offset.x()) * self.pixel_delta_u)
            + ((j as f32 + offset.y()) * self.pixel_delta_v);
        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square(&self) -> Vec3 {
        // Returns the vector to a random point in the [-.5, -.5] - [+.5, +.5] unit square.
        Vec3::new(rand_f32() - 0.5, rand_f32() - 0.5, 0.0)
    }
    fn ray_color(&self, r: &Ray, world: &HittableList) -> Vec3 {
        let mut rec = HitRecord::default();

        if world.hit(r, Interval::new(0.0, f32::INFINITY), &mut rec) {
            return (rec.normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5;
        }

        let unit_direction = r.direction().normalize();
        let a = 0.5 * (unit_direction.y() + 1.0);

        (Vec3::new(1.0, 1.0, 1.0) * (1.0 - a)) + (Vec3::new(0.5, 0.7, 1.0) * a)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            aspect_ratio: 16.0 / 9.0,
            image_width: 100,
            samples_per_pixel: 10,
            pixel_samples_scale: f32::default(),
            image_height: i32::default(),
            center: Vec3::default(),
            upper_left_pixel_loc: Vec3::default(),
            pixel_delta_u: Vec3::default(),
            pixel_delta_v: Vec3::default(),
        }
    }
}
