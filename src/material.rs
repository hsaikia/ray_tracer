use glam::f64::DVec3;
use rand::rngs::ThreadRng;
use rand::Rng;

type Color = DVec3;

use crate::Ray;

pub trait Material {
    fn get_reflected_ray(&self, pt: &DVec3, normal: &DVec3, rng: &mut ThreadRng) -> Ray;
    fn ambient_color(&self) -> Color;
}

pub struct Lambertian {
    pub ambient_color: Color,
}

impl Material for Lambertian {
    fn get_reflected_ray(&self, pt: &DVec3, normal: &DVec3, rng: &mut ThreadRng) -> Ray {
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
    fn ambient_color(&self) -> Color {
        self.ambient_color
    }
}
