use crate::{material::Material, vec3::Vec3};

#[derive(Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub mat: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, mat: Material) -> Self {
        Sphere {
            center,
            radius: radius.max(0.0),
            mat,
        }
    }
}
