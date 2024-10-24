use crate::{
    hittable::HitRecord,
    ray::Ray,
    util::rand_f32,
    vec3::{dot, random_vec, reflect, refract, Vec3},
};

#[derive(Clone, Copy)]
pub enum Material {
    Lambartian { albedo: Vec3 },
    Metal { albedo: Vec3, fuzz: f32 },
    // Refractive index in vacuum of air
    // Or the ratio of the refractive index over the refractive index of the enclosing media
    Dialetric { refraction_index: f32 },
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
                self.scatter_lambartian(*albedo, r_in, rec, attenuation, scattered)
            }

            Material::Metal { albedo, fuzz } => {
                self.scatter_metal(*albedo, *fuzz, *r_in, *rec, attenuation, scattered)
            }

            Material::Dialetric { refraction_index } => {
                self.scatter_dialetric(*refraction_index, *r_in, *rec, attenuation, scattered)
            }
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

    fn scatter_dialetric(
        &self,
        refraction_index: f32,
        r_in: Ray,
        rec: HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Vec3::new(1.0, 1.0, 1.0);
        let ri = if rec.front_face {
            1.0 / refraction_index
        } else {
            refraction_index
        };
        let unit_direction = r_in.direction().normalize();
        let cos_theta = dot(&-unit_direction, &rec.normal).min(1.0);
        let sin_theta = f32::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract = ri * sin_theta > 1.0;

        let direction = if cannot_refract || reflectance(cos_theta, ri) > rand_f32() {
            reflect(&unit_direction, &rec.normal)
        } else {
            refract(&unit_direction, &rec.normal, ri)
        };
        *scattered = Ray::new(rec.p, direction);
        true
    }
}

fn reflectance(cosine: f32, refraction_index: f32) -> f32 {
    // Use schlick's approximation for reflactance.
    let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * f32::powf(1.0 - cosine, 5.0)
}

impl Default for Material {
    fn default() -> Self {
        Material::Lambartian {
            albedo: Vec3::default(),
        }
    }
}
