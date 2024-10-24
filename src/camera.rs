use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};

use crate::{
    color::write_color,
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    interval::Interval,
    ray::Ray,
    util::rand_f32,
    vec3::{cross, rand_in_unit_disk},
    Vec3,
};

pub struct Camera {
    pub aspect_ratio: f32,      // Ratio of image width over height
    pub image_width: i32,       // Rendered image width in pixel count
    pub samples_per_pixel: i32, // Count of random samples for each pixel
    pub max_depth: u32,         // Maximum number of ray bounces into the scene
    pub vfov: f32,              // Vertical view angle (field of view)
    pub look_from: Vec3,        // Point camera is looking from
    pub look_at: Vec3,          // Point camera is looking at
    pub vup: Vec3,              // Camera-relative "up" direction
    pub defocus_angle: f32,     // Variation angle of rays through each pixel
    pub focus_dist: f32,        // Distance from camera lookfrom point to plane of perfect focus

    image_height: i32,          // Rendered image height
    pixel_samples_scale: f32,   // Color scale factor for a sum of pixel samples
    center: Vec3,               // Camera center
    upper_left_pixel_loc: Vec3, // Location of pixel (0, 0)
    pixel_delta_u: Vec3,        // Offset to pixel to the right
    pixel_delta_v: Vec3,        // Offset to pixel below

    // Camera frame basis vectors
    u: Vec3,
    v: Vec3,
    w: Vec3,

    defocus_disk_u: Vec3, // Defocus disk horizontal radius
    defocus_disk_v: Vec3, // Defocus disk vertical radius
}

impl Camera {
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
                pixel_color += self.ray_color(&r, self.max_depth, world);
            }
            write_color(&(pixel_color * self.pixel_samples_scale));
        }
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f32 / self.aspect_ratio) as i32;
        assert!(self.image_height > 1);

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f32;

        self.center = self.look_from;

        // Determine viewport dimensions.
        let theta = self.vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * ((self.image_width / self.image_height) as f32);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        self.w = (self.look_from - self.look_at).normalize();
        self.u = cross(&self.vup, &self.w).normalize();
        self.v = cross(&self.w, &self.u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * self.u; // Vector across the viewport horizontal edge
        let viewport_v = viewport_height * -self.v; // Vector down the viewport vertical edge

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width;
        self.pixel_delta_v = viewport_v / self.image_height;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            self.center - (self.focus_dist * self.w) - viewport_u / 2.0 - viewport_v / 2.0;

        self.upper_left_pixel_loc =
            viewport_upper_left + ((self.pixel_delta_u + self.pixel_delta_v) * 0.5);

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = self.focus_dist * (self.defocus_angle / 2.0).to_radians().tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        // Construct a camera ray originating from the defocus disk directed at randomly sampled
        // point around the pixel location (i, j).
        let offset = self.sample_square();
        let pixel_sample = self.upper_left_pixel_loc
            + ((i as f32 + offset.x()) * self.pixel_delta_u)
            + ((j as f32 + offset.y()) * self.pixel_delta_v);
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square(&self) -> Vec3 {
        // Returns the vector to a random point in the [-.5, -.5] - [+.5, +.5] unit square.
        Vec3::new(rand_f32() - 0.5, rand_f32() - 0.5, 0.0)
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        // Returns a random point in the camera defocus disk.
        let p = rand_in_unit_disk();
        self.center + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v)
    }

    fn ray_color(&self, r: &Ray, depth: u32, world: &HittableList) -> Vec3 {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth <= 0 {
            return Vec3::default();
        }

        let mut rec = HitRecord::default();

        if world.hit(r, Interval::new(0.001, f32::INFINITY), &mut rec) {
            let mut scattered = Ray::default();
            let mut attenuation = Vec3::default();

            if rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
                return attenuation * self.ray_color(&scattered, depth - 1, world);
            }

            return Vec3::default();
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
            max_depth: 10,
            vfov: f32::default(),
            vup: Vec3::default(),
            look_at: Vec3::default(),
            look_from: Vec3::default(),
            u: Vec3::default(),
            w: Vec3::default(),
            v: Vec3::default(),
            pixel_samples_scale: f32::default(),
            image_height: i32::default(),
            center: Vec3::default(),
            upper_left_pixel_loc: Vec3::default(),
            pixel_delta_u: Vec3::default(),
            pixel_delta_v: Vec3::default(),
            defocus_angle: f32::default(),
            focus_dist: f32::default(),
            defocus_disk_u: Vec3::default(),
            defocus_disk_v: Vec3::default(),
        }
    }
}
