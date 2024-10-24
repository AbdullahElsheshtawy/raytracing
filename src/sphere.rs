use crate::{hittable::Hittable, interval::Interval, material::Material, vec3::dot, Vec3};

pub struct Sphere {
    center: Vec3,
    radius: f32,
    mat: Material,
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

impl Hittable for Sphere {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        ray_t: Interval,
        rec: &mut crate::hittable::HitRecord,
    ) -> bool {
        let oc = self.center - *r.origin();
        let a = r.direction().length_squared();
        let h = dot(r.direction(), &oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;

            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.mat = self.mat;
        true
    }
}
