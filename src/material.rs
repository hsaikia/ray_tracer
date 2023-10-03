use glam::f64::DVec3;
use rand::rngs::ThreadRng;
use rand::Rng;

type Color = DVec3;

pub trait Material {
    fn get_new_dir(&self, incident_dir: &DVec3, normal: &DVec3, rng: &mut ThreadRng) -> DVec3;
    fn reflectance(&self, incident_color: &Color) -> Color;
}

pub struct Lambertian {
    pub ambient_color: Color,
    pub reflectance_factor: f64,
}

impl Material for Lambertian {
    fn get_new_dir(&self, _incident_dir: &DVec3, normal: &DVec3, rng: &mut ThreadRng) -> DVec3 {
        let mut dir = DVec3 {
            x: rng.gen_range(-1.0..1.0),
            y: rng.gen_range(-1.0..1.0),
            z: rng.gen_range(-1.0..1.0),
        };

        // Should be reflected by the outer surface
        if dir.dot(*normal) < 0.0 {
            dir = -dir;
        }

        // True Lambertian reflectance -> add the normal
        dir += *normal;

        // Normalize
        dir = dir.normalize();

        dir
    }

    fn reflectance(&self, incident_color: &Color) -> Color {
        self.ambient_color * (1.0 - self.reflectance_factor)
            + self.reflectance_factor * *incident_color
    }
}

pub struct Metal {
    pub ambient_color: Color,
}

impl Material for Metal {
    fn get_new_dir(&self, incident_dir: &DVec3, normal: &DVec3, _rng: &mut ThreadRng) -> DVec3 {
        let d_i = incident_dir.normalize();
        let n = normal.normalize();

        // Perfect mirror reflection
        d_i - 2.0 * n * (n.dot(d_i))
    }

    fn reflectance(&self, incident_color: &Color) -> Color {
        self.ambient_color * *incident_color
    }
}

#[derive(Clone)]
pub struct Dielectric {
    pub ambient_color: Color,
    pub refraction_coeff: f64,
}

impl Material for Dielectric {
    fn get_new_dir(&self, incident_dir: &DVec3, normal: &DVec3, _rng: &mut ThreadRng) -> DVec3 {
        // Dielectrics would refract or reflect
        let d_i = incident_dir.normalize();
        let n = normal.normalize();

        // Travelling from lower refractive index to higher (air to water)
        let mut frac = 1.0 / self.refraction_coeff;
        let cosine_incidence = d_i.dot(n);
        let sine_incidence = (1.0 - cosine_incidence * cosine_incidence).sqrt();

        if cosine_incidence > 0.0 {
            // Travelling from higher refractive index to lower (water to air)
            // Ray is travelling from inside the dielectric to the surface
            frac = 1.0 / frac;
            let sine_refracted = sine_incidence * frac;
            if sine_refracted > 1.0 {
                // Total internal reflection
                return d_i - 2.0 * n * cosine_incidence;
            } else {
                let cosine_refracted = (1.0 - sine_refracted * sine_refracted).sqrt();
                return cosine_refracted * n + frac * (d_i - cosine_incidence * n);
            }
        }

        // Otherwise, refract
        let sine_refracted = sine_incidence * frac;
        let cosine_refracted = (1.0 - sine_refracted * sine_refracted).sqrt();
        -cosine_refracted * n + frac * (d_i + cosine_incidence * n)
    }
    fn reflectance(&self, incident_color: &Color) -> Color {
        self.ambient_color * *incident_color
    }
}
