use glam::f64::DVec3;
use indicatif::ProgressBar;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::fs::File;
use std::io::BufWriter;

const NUM_SAMPLE_RAYS: usize = 32;
const HEIGHT_PIXELS: usize = 512;
const WIDTH_PIXELS: usize = 512;
const TOTAL_SIZE: usize = WIDTH_PIXELS * HEIGHT_PIXELS * 3;
const HEIGHT: f64 = 20.0;
const WIDTH: f64 = 20.0;
const PIXEL_WIDTH: f64 = WIDTH / WIDTH_PIXELS as f64;
const PIXEL_HEIGHT: f64 = HEIGHT / HEIGHT_PIXELS as f64;
const CAMERA_Z: f64 = 8.0;
const MAX_DEPTH: i32 = 10;

type Color = DVec3;

mod material;
mod ray;
mod renderables;

use material::{Dielectric, Lambertian, Metal};
use ray::Ray;
use renderables::{Renderable, Sphere};

const BACKGROUND_COLOR: DVec3 = DVec3 {
    x: 0.8,
    y: 1.0,
    z: 1.0,
};

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
            let new_ray = rend.get_new_ray(&x, &ray.direction, rng);
            let incident_color = intersection(&new_ray, scene, rng, depth + 1);
            return rend.reflectance_color(&incident_color);
        }
    }

    BACKGROUND_COLOR
}

fn random_ray(x: usize, y: usize, camera_z: f64, rng: &mut ThreadRng) -> Ray {
    let dx = rng.gen_range(-PIXEL_WIDTH / 2.0..PIXEL_WIDTH / 2.0);
    let dy = rng.gen_range(-PIXEL_HEIGHT / 2.0..PIXEL_HEIGHT / 2.0);
    let x = (((x as i64 - WIDTH_PIXELS as i64 / 2) as f64) / WIDTH_PIXELS as f64) * WIDTH + dx;
    let y = (((y as i64 - HEIGHT_PIXELS as i64 / 2) as f64) / HEIGHT_PIXELS as f64) * HEIGHT + dy;

    let mut direction: DVec3 = DVec3 { x, y, z: -camera_z };
    direction = direction.normalize();
    let origin: DVec3 = DVec3 {
        x: 0.0,
        y: 0.0,
        z: camera_z,
    };
    Ray { origin, direction }
}

fn main() {
    // Setup materials
    let blue_material = Box::new(Lambertian {
        ambient_color: DVec3 {
            x: 0.0,
            y: 0.5,
            z: 1.0,
        },
        reflectance_factor: 0.5,
    });

    let silver_material = Box::new(Metal {
        ambient_color: DVec3 {
            x: 0.8,
            y: 0.8,
            z: 0.8,
        },
    });

    let glass_material = Box::new(Dielectric {
        ambient_color: DVec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
        refraction_coeff: 1.5,
    });

    let green_material = Box::new(Lambertian {
        ambient_color: DVec3 {
            x: 0.2,
            y: 0.7,
            z: 0.1,
        },
        reflectance_factor: 0.5,
    });

    // Setup scene
    let scene: Vec<Box<dyn Renderable>> = vec![
        // Spheres
        Box::new(Sphere::new(
            DVec3 {
                x: 0.0,
                y: 0.0,
                z: -8.0,
            },
            8.0,
            blue_material,
        )),
        Box::new(Sphere::new(
            DVec3 {
                x: 16.0,
                y: 0.0,
                z: -8.0,
            },
            8.0,
            silver_material,
        )),
        Box::new(Sphere::new(
            DVec3 {
                x: -16.0,
                y: 0.0,
                z: -8.0,
            },
            8.0,
            glass_material,
        )),
        // Earth
        Box::new(Sphere::new(
            DVec3 {
                x: 0.0,
                y: 1000.0,
                z: -8.0,
            },
            992.0,
            green_material,
        )),
    ];

    // Progress Bar
    let bar = ProgressBar::new(WIDTH_PIXELS as u64 * HEIGHT_PIXELS as u64 * NUM_SAMPLE_RAYS as u64);

    // RNG
    let mut rng = rand::thread_rng();

    // Set up image file
    let path = "image.png";
    let file = File::create(path).unwrap();
    let w = &mut BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, WIDTH_PIXELS as u32, HEIGHT_PIXELS as u32);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    let mut data: [u8; TOTAL_SIZE] = [0; TOTAL_SIZE];
    // Draw
    for y in 0..HEIGHT_PIXELS {
        for x in 0..WIDTH_PIXELS {
            let mut col = DVec3::ZERO;
            for _ in 0..NUM_SAMPLE_RAYS {
                let ray = random_ray(x, y, CAMERA_Z, &mut rng);
                col += intersection(&ray, &scene, &mut rng, 0);
                bar.inc(1);
            }
            col /= NUM_SAMPLE_RAYS as f64;
            let idx = y * WIDTH_PIXELS + x;
            data[3 * idx] = (col.x * 255.0) as u8;
            data[3 * idx + 1] = (col.y * 255.0) as u8;
            data[3 * idx + 2] = (col.z * 255.0) as u8;
        }
    }
    writer.write_image_data(&data).unwrap();
}
