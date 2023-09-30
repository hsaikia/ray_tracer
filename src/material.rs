use glam::f64::DVec3;
use rand::rngs::ThreadRng;
use rand::Rng;

type Color = DVec3;

use crate::Ray;

pub trait Material {
    fn get_reflected_ray(
        &self,
        pt: &DVec3,
        incident_dir: &DVec3,
        normal: &DVec3,
        rng: &mut ThreadRng,
    ) -> Ray;
    fn reflectance(&self, incident_color: &Color) -> Color;
}

pub struct Lambertian {
    pub ambient_color: Color,
    pub reflectance_factor: f64,
}

impl Material for Lambertian {
    fn get_reflected_ray(
        &self,
        pt: &DVec3,
        _incident_dir: &DVec3,
        normal: &DVec3,
        rng: &mut ThreadRng,
    ) -> Ray {
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

        Ray {
            origin: *pt,
            direction: dir,
        }
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
    fn get_reflected_ray(
        &self,
        pt: &DVec3,
        incident_dir: &DVec3,
        normal: &DVec3,
        _rng: &mut ThreadRng,
    ) -> Ray {
        let d_i = incident_dir.normalize();
        let n = normal.normalize();

        // Perfect mirror reflection
        Ray {
            origin: *pt,
            direction: d_i - 2.0 * n * (n.dot(d_i)),
        }
    }

    fn reflectance(&self, incident_color: &Color) -> Color {
        //self.ambient_color  * (DVec3::ONE * 0.5 + *incident_color * 0.5)
        self.ambient_color * *incident_color
    }
}
