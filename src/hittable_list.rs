use crate::hittable::{HitRecord, Hittable};
use std::rc::Rc;

pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new(object: Rc<dyn Hittable>) -> Self {
        let mut hl = Self::default();
        hl.add(object);
        hl
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        ray_tmin: f32,
        ray_tmax: f32,
        rec: &mut crate::hittable::HitRecord,
    ) -> bool {
        let mut temp_record = HitRecord::default();

        let mut hit_anything = false;
        let mut closest_so_far = ray_tmax;

        for object in &self.objects {
            if object.hit(r, ray_tmin, closest_so_far, &mut temp_record) {
                hit_anything = true;
                closest_so_far = temp_record.t;
                *rec = temp_record;
            }
        }
        hit_anything
    }
}

impl Default for HittableList {
    fn default() -> Self {
        HittableList {
            objects: Vec::default(),
        }
    }
}
