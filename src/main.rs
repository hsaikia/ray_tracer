use glam::f64::DVec3;
use glam::UVec3;
use indicatif::ProgressBar;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::fs::File;
use std::io::Error;
use std::io::LineWriter;
use std::io::Write;

const NUM_SAMPLE_RAYS: i32 = 32;
const HEIGHT_PIXELS: i32 = 512;
const WIDTH_PIXELS: i32 = 512;
const HEIGHT: f64 = 20.0;
const WIDTH: f64 = 20.0;
const PIXEL_WIDTH: f64 = WIDTH / WIDTH_PIXELS as f64;
const PIXEL_HEIGHT: f64 = HEIGHT / HEIGHT_PIXELS as f64;
const COLORS: usize = 255;
const CAMERA_Z: f64 = 5.0;
const MAX_DEPTH: i32 = 4;
const REFLECTED_RADIANCE_FRACTION: f64 = 0.5;

type Color = DVec3;

const BACKGROUND_COLOR: DVec3 = DVec3 {
    x: 0.8,
    y: 1.0,
    z: 1.0,
};

struct Ray {
    origin: DVec3,
    direction: DVec3,
}

impl Ray {
    fn eval(&self, t: f64) -> DVec3 {
        self.origin + t * self.direction
    }
}

struct Sphere {
    center: DVec3,
    radius: f64,
    ambient_color: Color,
}

impl Sphere {
    fn new(center: DVec3, radius: f64, ambient_color: Color) -> Self {
        Sphere {
            center,
            radius,
            ambient_color,
        }
    }
}

trait Renderable {
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
        self.ambient_color
    }
    fn normal(&self, point: &DVec3) -> DVec3 {
        (*point - self.center).normalize()
    }

    fn get_reflected_ray(&self, pt: &DVec3, rng: &mut ThreadRng) -> Ray {
        let mut dir = DVec3 {
            x: rng.gen_range(-1.0..1.0),
            y: rng.gen_range(-1.0..1.0),
            z: rng.gen_range(-1.0..1.0),
        };
        dir = dir.normalize();
        if dir.dot(self.normal(pt)) < 0.0 {
            dir = -dir;
        }
        Ray {
            origin: *pt,
            direction: dir,
        }
    }
}

fn intersection(
    ray: &Ray,
    scene: &Vec<Box<dyn Renderable>>,
    rng: &mut ThreadRng,
    depth: i32,
) -> Color {
    if depth == MAX_DEPTH {
        return DVec3::ZERO;
    }

    for rend in scene {
        if let Some(x) = rend.intersect(ray) {
            // get random reflected ray
            let reflected_ray = rend.get_reflected_ray(&x, rng);
            return rend.ambient_color() * (1.0 - REFLECTED_RADIANCE_FRACTION)
                + REFLECTED_RADIANCE_FRACTION
                    * intersection(&reflected_ray, scene, rng, depth + 1);
        }
    }

    BACKGROUND_COLOR
}

fn random_ray(x: i32, y: i32, camera_z: f64, rng: &mut ThreadRng) -> Ray {
    let dx = rng.gen_range(-PIXEL_WIDTH / 2.0..PIXEL_WIDTH / 2.0);
    let dy = rng.gen_range(-PIXEL_HEIGHT / 2.0..PIXEL_HEIGHT / 2.0);
    let x = (((x - WIDTH_PIXELS / 2) as f64) / WIDTH_PIXELS as f64) * WIDTH + dx;
    let y = (((y - HEIGHT_PIXELS / 2) as f64) / HEIGHT_PIXELS as f64) * HEIGHT + dy;

    let mut direction: DVec3 = DVec3 { x, y, z: -camera_z };
    direction = direction.normalize();
    let origin: DVec3 = DVec3 {
        x: 0.0,
        y: 0.0,
        z: camera_z,
    };
    Ray { origin, direction }
}

fn color_to_string(color: &DVec3) -> String {
    //println!("{:?}", color);
    assert!(color.x >= 0.0 && color.x <= 1.0);
    assert!(color.y >= 0.0 && color.y <= 1.0);
    assert!(color.z >= 0.0 && color.z <= 1.0);

    let col: UVec3 = UVec3 {
        x: (color.x * 255.0) as u32,
        y: (color.y * 255.0) as u32,
        z: (color.z * 255.0) as u32,
    };
    format!("{} {} {}\n", col.x, col.y, col.z)
}

fn main() -> Result<(), Error> {
    let path = "image.ppm";
    let file = File::create(path)?;
    let mut file = LineWriter::new(file);

    let bar = ProgressBar::new(WIDTH_PIXELS as u64 * HEIGHT_PIXELS as u64 * NUM_SAMPLE_RAYS as u64);
    let header = format!("P3\n{WIDTH_PIXELS} {HEIGHT_PIXELS}\n{COLORS}\n");
    file.write_all(header.as_bytes())?;

    // Setup scene
    let scene: Vec<Box<dyn Renderable>> = vec![
        // Sphere
        Box::new(Sphere::new(
            DVec3 {
                x: 0.0,
                y: 0.0,
                z: -8.0,
            },
            8.0,
            DVec3 {
                x: 0.0,
                y: 0.5,
                z: 1.0,
            },
        )),
        // Earth
        Box::new(Sphere::new(
            DVec3 {
                x: 0.0,
                y: 1000.0,
                z: -8.0,
            },
            992.0,
            DVec3 {
                x: 0.1,
                y: 0.7,
                z: 0.1,
            },
        )),
    ];

    // Draw
    let mut rng = rand::thread_rng();
    for y in 0..HEIGHT_PIXELS {
        for x in 0..WIDTH_PIXELS {
            let mut col = DVec3::ZERO;
            for _ in 0..NUM_SAMPLE_RAYS {
                let ray = random_ray(x, y, CAMERA_Z, &mut rng);
                col += intersection(&ray, &scene, &mut rng, 0);
                bar.inc(1);
            }
            col /= NUM_SAMPLE_RAYS as f64;
            // Have to be all integers
            let col_str = color_to_string(&col);
            file.write_all(col_str.as_bytes())?;
        }
    }
    Ok(())
}
