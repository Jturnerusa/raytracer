#![allow(dead_code, unused_imports)]
mod camera;
mod frame;
mod hit;
mod ray;
mod sphere;

use camera::Camera;
use clap::Parser;
use core::ops::Range;
use frame::{FrameBuffer, Rgba32};
use hit::Hit;
use nalgebra::{ComplexField, Vector3};
use rand::rngs::OsRng;
use rand::Rng;
use ray::Ray;
use rayon::iter::{
    IndexedParallelIterator, IntoParallelIterator, IntoParallelRefMutIterator, ParallelIterator,
};
use rayon::slice::ParallelSliceMut;
use sphere::Sphere;
use std::error::Error;
use std::io::{self, Write};
use std::iter;
use std::ops::Deref;

const ASPECT_RATIO: f64 = 16.0 / 9.0;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    width: usize,
    #[arg(long)]
    samples: usize,
    #[arg(long)]
    bounces: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut frame_buffer = FrameBuffer::new(args.width, args.width / ASPECT_RATIO as usize);
    let spheres = [
        Sphere {
            center: Vector3::new(0.0, 0.0, -1.0),
            radius: 0.5,
        },
        Sphere {
            center: Vector3::new(0.0, -100.5, -1.0),
            radius: 100.0,
        },
    ];
    let camera = Camera::new(
        Vector3::new(0.0, 0.0, 1.0),
        ASPECT_RATIO,
        1.0,
        args.width as u64,
    );
    draw_scene(
        camera,
        spheres.as_slice(),
        &mut frame_buffer,
        args.samples,
        args.bounces,
    )?;
    write_ppm(
        frame_buffer.width(),
        frame_buffer.height(),
        frame_buffer.pixel_data(),
        &mut io::stdout(),
    )?;
    Ok(())
}

fn draw_scene(
    camera: Camera,
    spheres: &[Sphere],
    frame_buffer: &mut FrameBuffer,
    samples: usize,
    bounces: usize,
) -> Result<(), Box<dyn Error>> {
    let width = frame_buffer.width();
    let height = frame_buffer.height();
    frame_buffer
        .pixel_data_mut()
        .par_chunks_mut(4)
        .enumerate()
        .for_each(|(index, data)| {
            let x = index % width;
            let y = index / height;
            let color =
                iter::repeat_with(|| camera.cast(x as f64, y as f64, OsRng.gen_range(0.0..1.0)))
                    .map(|ray| ray_color(spheres, ray, None, bounces))
                    .take(samples)
                    .reduce(|acc, e| acc + e)
                    .unwrap()
                    / samples as f64;
            let (r, g, b, a) = color.to_rgba32();
            data.copy_from_slice(&[r, g, b, a]);
        });

    Ok(())
}

fn ray_color(spheres: &[Sphere], ray: Ray, skip: Option<Sphere>, bounces: usize) -> Vector3<f64> {
    for (i, sphere) in spheres.iter().copied().enumerate() {
        if skip.is_some_and(|s| sphere == s) {
            continue;
        }
        match sphere.hit(ray) {
            Some(hit) => {
                let random = loop {
                    let random = Vector3::new(
                        OsRng.gen_range(-1.0..1.0),
                        OsRng.gen_range(-1.0..1.0),
                        OsRng.gen_range(-1.0..1.0),
                    );
                    if random.dot(&random) <= 1.0 {
                        break random / random.dot(&random).sqrt();
                    } else {
                        continue;
                    };
                };

                let direction = hit.normal + random;
                if bounces > 0 {
                    return 0.7
                        * ray_color(
                            spheres,
                            Ray {
                                origin: hit.point,
                                direction,
                            },
                            Some(sphere),
                            bounces - 1,
                        );
                } else {
                    break;
                }
            }
            None => continue,
        }
    }
    let unit_direction = ray.direction.y / ray.direction.dot(&ray.direction).sqrt();
    let a = 0.5 * (unit_direction + 1.0);
    (1.0 - a) * Vector3::new(1.0, 1.0, 1.0) + a * Vector3::new(0.5, 0.7, 1.0)
}

fn write_ppm(width: usize, height: usize, data: &[u8], mut writer: impl Write) -> io::Result<()> {
    writeln!(writer, "P3")?;
    writeln!(writer, "{width} {height}")?;
    writeln!(writer, "255")?;
    for [r, g, b, _] in data
        .chunks_exact(4)
        .map(|chunk| <[u8; 4]>::try_from(chunk).unwrap())
    {
        writeln!(writer, "{r} {g} {b}")?;
    }
    Ok(())
}
