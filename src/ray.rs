use glam::f64::DVec3;

pub struct Ray {
    pub origin: DVec3,
    pub direction: DVec3,
}

impl Ray {
    pub fn eval(&self, t: f64) -> DVec3 {
        self.origin + t * self.direction
    }
}
