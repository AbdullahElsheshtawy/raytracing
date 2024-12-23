use crate::{material::Material, ray::Ray, vec3::dot, Vec3};

#[derive(Clone, Copy, Default)]
pub struct HitRecord {
    pub front_face: bool,
    pub mat: Material,
    pub normal: Vec3,
    pub p: Vec3,
    pub t: f32,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter 'outward_normal' is assumed to have unit_length

        self.front_face = dot(r.direction(), &outward_normal) < 0.0;

        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}
