use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
    sphere::Sphere,
    vec3::dot,
};

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<HittableObject>,
}

#[derive(Clone)]
pub enum HittableObject {
    Sphere(Sphere),
}

impl HittableObject {
    pub fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        match self {
            HittableObject::Sphere(sphere) => Self::sphere_hit(sphere, r, ray_t, rec),
        }
    }

    fn sphere_hit(sphere: &Sphere, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc = sphere.center - *r.origin();
        let a = r.direction().length_squared();
        let h = dot(r.direction(), &oc);
        let c = oc.length_squared() - sphere.radius * sphere.radius;

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
        let outward_normal = (rec.p - sphere.center) / sphere.radius;
        rec.set_face_normal(r, outward_normal);
        rec.mat = sphere.mat;
        true
    }
}
impl HittableList {
    pub fn add(&mut self, object: HittableObject) {
        self.objects.push(object);
    }

    pub fn hit(
        &self,
        r: &crate::ray::Ray,
        ray_t: Interval,
        rec: &mut crate::hittable::HitRecord,
    ) -> bool {
        let mut temp_record = HitRecord::default();

        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if object.hit(
                r,
                Interval::new(ray_t.min, closest_so_far),
                &mut temp_record,
            ) {
                hit_anything = true;
                closest_so_far = temp_record.t;
                *rec = temp_record;
            }
        }
        hit_anything
    }
}

impl Hittable for HittableList {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        ray_t: Interval,
        rec: &mut crate::hittable::HitRecord,
    ) -> bool {
        let mut temp_record = HitRecord::default();

        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if object.hit(
                r,
                Interval::new(ray_t.min, closest_so_far),
                &mut temp_record,
            ) {
                hit_anything = true;
                closest_so_far = temp_record.t;
                *rec = temp_record;
            }
        }
        hit_anything
    }
}
