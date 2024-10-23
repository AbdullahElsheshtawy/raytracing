use crate::{
    hittable::HitRecord,
    ray::Ray,
    vec3::{dot, random_vec, reflect, Vec3},
};

#[derive(Clone, Copy)]
pub enum Material {
    Lambartian { albedo: Vec3 },
    Metal { albedo: Vec3, fuzz: f32 },
}

impl Material {
    pub fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        match self {
            Material::Lambartian { albedo } => {
                self.scatter_lambartian(*albedo, r_in, rec, &mut *attenuation, &mut *scattered)
            }

            Material::Metal { albedo, fuzz } => self.scatter_metal(
                *albedo,
                *fuzz,
                *r_in,
                *rec,
                &mut *attenuation,
                &mut *scattered,
            ),
        }
    }

    fn scatter_lambartian(
        &self,
        albedo: Vec3,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + random_vec();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = albedo;
        true
    }

    fn scatter_metal(
        &self,
        albedo: Vec3,
        fuzz: f32,
        r_in: Ray,
        rec: HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let mut reflected = reflect(r_in.direction(), &rec.normal);
        reflected = reflected.normalize() + (fuzz * random_vec());
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = albedo;
        return dot(scattered.direction(), &rec.normal) > 0.0;
    }
}

impl Default for Material {
    fn default() -> Self {
        Material::Lambartian {
            albedo: Vec3::default(),
        }
    }
}
