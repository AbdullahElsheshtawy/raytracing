use crate::{dot, interval::Interval, ray::Ray, Vec3};

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f32,
    front_face: bool,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter 'outward_normal' is assumed to have unit_length

        self.front_face = dot(r.direction(), &outward_normal) < 0.0;

        self.normal = match self.front_face {
            true => outward_normal,
            false => -outward_normal,
        };
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        HitRecord {
            p: Vec3::default(),
            normal: Vec3::default(),
            t: f32::default(),
            front_face: bool::default(),
        }
    }
}
