#![allow(dead_code, unused_imports)]

use nalgebra::Vector3;
use rand::Rng;
use std::{error::Error, io::Write};

mod camera;
mod frame;
mod ray;
mod sphere;

use camera::Camera;
use frame::FrameBuffer;
use ray::Ray;
use sphere::Sphere;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: usize = 1920 / 2 - 5;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
const SAMPLES: usize = 20;

fn main() -> Result<(), Box<dyn Error>> {
    let sdl2_context = sdl2::init()?;
    let mut canvas = sdl2_context
        .video()?
        .window("raytracer", IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32)
        .position_centered()
        .build()?
        .into_canvas()
        .build()?;
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator.create_texture_streaming(
        sdl2::pixels::PixelFormatEnum::RGBA32,
        IMAGE_WIDTH as u32,
        IMAGE_HEIGHT as u32,
    )?;
    let mut events = sdl2_context.event_pump()?;

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
        spheres.iter().copied(),
        &mut frame_buffer,
        SAMPLES,
        &mut rng,
    )?;

    texture.update(
        sdl2::rect::Rect::new(0, 0, IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32),
        frame_buffer.pixel_data(),
        IMAGE_WIDTH * 4,
    )?;
    canvas.copy(&texture, None, None)?;
    canvas.present();

    'main: loop {
        for event in events.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => continue,
            }
        }
    }
    Ok(())
}

fn draw_scene(
    camera: Camera,
    spheres: impl Iterator<Item = Sphere>,
    frame_buffer: &mut FrameBuffer,
    samples: usize,
    rng: &mut rand::rngs::OsRng,
) -> Result<(), Box<dyn Error>> {
    for sphere in spheres {
        for y in 0..frame_buffer.height() {
            for x in 0..frame_buffer.width() {
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
            let lerp = (1.0 - a) * blue + a * white;
            let color = sdl2::pixels::Color::RGBA(
                (lerp.x * 255.0) as u8,
                (lerp.y * 255.0) as u8,
                (lerp.z * 255.0) as u8,
                0,
            );
            frame_buffer.set_pixel(x, y, color);
        }
    }
}
