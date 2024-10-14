#![allow(dead_code, unused_imports)]
mod camera;
mod frame;
mod ray;
mod sphere;

use camera::Camera;
use frame::FrameBuffer;
use nalgebra::Vector3;
use rand::Rng;
use ray::Ray;
use sphere::Sphere;
use std::error::Error;
use std::io::{self, Write};

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: usize = 1920 / 2 - 5;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
const SAMPLES: usize = 20;

fn main() -> Result<(), Box<dyn Error>> {
    let mut frame_buffer = FrameBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    let mut rng = rand::rngs::OsRng;
    let spheres = [Sphere {
        center: Vector3::new(0.0, 0.0, 0.0),
        radius: 0.25,
    }];
    let camera = Camera::new(
        Vector3::new(0.0, 0.5, -2.0),
        ASPECT_RATIO,
        1.0,
        IMAGE_WIDTH as u64,
    );
    draw_background(&mut frame_buffer);
    draw_scene(
        camera,
        spheres.as_slice(),
        &mut frame_buffer,
        SAMPLES,
        &mut rng,
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
    rng: &mut rand::rngs::OsRng,
) -> Result<(), Box<dyn Error>> {
    for y in 0..frame_buffer.height() {
        for x in 0..frame_buffer.width() {
            for sphere in spheres {
                let color = std::iter::repeat_with(|| camera.cast(x as f64, y as f64, rng.gen()))
                    .map(|ray| {
                        if sphere.intersects(ray) {
                            Vector3::new(0.5, 0.5, 1.0)
                        } else {
                            frame_buffer.get_pixel(x, y)
                        }
                    })
                    .take(samples)
                    .reduce(|acc, e| acc + e)
                    .unwrap()
                    / SAMPLES as f64;
                frame_buffer.set_pixel(x, y, color);
            }
        }
    }
    Ok(())
}

fn draw_background(frame_buffer: &mut FrameBuffer) {
    let white = Vector3::new(1.0, 1.0, 1.0);
    let blue = Vector3::new(0.5, 0.7, 1.0);
    for y in 0..frame_buffer.height() {
        for x in 0..frame_buffer.width() {
            let a = y as f64 / IMAGE_HEIGHT as f64;
            let color = (1.0 - a) * blue + a * white;
            frame_buffer.set_pixel(x, y, color);
        }
    }
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
