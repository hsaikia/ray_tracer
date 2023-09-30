use crate::material::Material;
use crate::Ray;
use glam::f64::DVec3;
use rand::rngs::ThreadRng;

pub struct Sphere {
    center: DVec3,
    radius: f64,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: DVec3, radius: f64, material: Box<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

pub trait Renderable {
    fn intersect(&self, ray: &Ray) -> Option<DVec3>;
    fn ambient_color(&self) -> DVec3;
    fn normal(&self, point: &DVec3) -> DVec3;
    fn get_reflected_ray(&self, pt: &DVec3, rng: &mut ThreadRng) -> Ray;
}

impl Renderable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<DVec3> {
        let y = ray.origin - self.center;
        let b = 2.0 * ray.direction.dot(y);
        let c = y.length_squared() - self.radius * self.radius;

        let det = b * b - 4.0 * c;
        if det < 0.0 {
            return None;
        }

        let det_root = det.sqrt();

        let t1 = (-b - det_root) / 2.0;
        let t2 = (-b + det_root) / 2.0;

        if t2 < 0.0 {
            return None;
        }

        if t1 < 0.0 {
            return Some(ray.eval(t2));
        }

        Some(ray.eval(t1))
    }

    fn ambient_color(&self) -> DVec3 {
        self.material.ambient_color()
    }
    fn normal(&self, point: &DVec3) -> DVec3 {
        (*point - self.center).normalize()
    }

    fn get_reflected_ray(&self, pt: &DVec3, rng: &mut ThreadRng) -> Ray {
        self.material.get_reflected_ray(pt, &self.normal(pt), rng)
    }
}