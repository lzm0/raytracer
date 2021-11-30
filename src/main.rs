mod camera;
mod color;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod utils;
mod vec3;

use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::thread;

pub use camera::*;
pub use color::*;
pub use hittable::*;
pub use hittable_list::*;
pub use ray::*;
pub use sphere::*;
pub use utils::*;
pub use vec3::*;

fn ray_color(r: Ray, world: &dyn Hittable, depth: i32) -> Color {
    let mut rec = HitRecord::new();

    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if world.hit(&r, 0.001, INFINITY, &mut rec) {
        let target = rec.p + rec.normal + random_unit_vector();
        return 0.5 * ray_color(Ray::new(rec.p, target - rec.p), world, depth - 1);
    }
    let unit_direction = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width: i32 = 400;
    let image_height: i32 = ((image_width as f32) / aspect_ratio) as i32;
    let samples_per_pixel: i32 = 512;
    let max_depth: i32 = 100;

    let concurrency = 8;

    let mut world = HittableList::new();
    world.add(Arc::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Arc::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let cam = Camera::new();

    print!("P3\n{} {}\n255\n", image_width, image_height);

    let (tx, rx): (Sender<i32>, Receiver<i32>) = channel();
    let mut handles = vec![];

    for cpu in (0..concurrency).rev() {
        let tx = tx.clone();
        let world = world.clone();
        handles.push(thread::spawn(move || {
            let mut pixels = vec![];
            for y in
                ((cpu * image_height / concurrency)..((cpu + 1) * image_height / concurrency)).rev()
            {
                for x in 0..image_width {
                    let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                    for _ in 0..samples_per_pixel {
                        let u: f32 = (x as f32 + random()) / (image_width as f32);
                        let v: f32 = (y as f32 + random()) / (image_height as f32);
                        let r = cam.get_ray(u, v);
                        pixel_color += ray_color(r, &world, max_depth);
                    }
                    pixels.push(pixel_color);
                }
                tx.send(1).unwrap();
            }
            pixels
        }));
    }

    for y in (0..image_height).rev() {
        rx.recv().unwrap();
        eprint!("\rScanlines remaining: {:>4}", y);
    }
    eprint!("\nDone\n");

    handles.into_iter().for_each(|t| {
        let pixels = t.join().unwrap();
        for pixel in pixels {
            write_color(pixel, samples_per_pixel);
        }
    });
}
