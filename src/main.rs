#![allow(dead_code)]
mod camera;
mod frame;
mod hit;
mod quad;
mod ray;
mod sphere;

use camera::Camera;
use clap::Parser;
use frame::{FrameBuffer, Rgba32};
use hit::{Hit, Material};
use nalgebra::{ComplexField, Vector3};
use quad::Quad;
use rand::rngs::OsRng;
use rand::Rng;
use ray::Ray;
use rayon::iter::{IndexedParallelIterator, ParallelIterator};
use rayon::slice::ParallelSliceMut;
use sphere::Sphere;
use std::error::Error;
use std::io::{self, Write};
use std::iter;

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

#[derive(Clone, Copy, Debug, PartialEq)]
enum Shape {
    Sphere(Sphere),
    Quad(Quad),
}

impl Hit for Shape {
    fn hit(&self, ray: Ray) -> Option<hit::Record> {
        match self {
            Self::Sphere(sphere) => sphere.hit(ray),
            Self::Quad(quad) => quad.hit(ray),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut frame_buffer = FrameBuffer::new(args.width, args.width / ASPECT_RATIO as usize);

    let red = Vector3::new(0.65, 0.05, 0.5);
    let white = Vector3::new(0.73, 0.73, 0.73);
    let green = Vector3::new(0.12, 0.45, 0.15);

    let shapes = [
        Shape::Sphere(Sphere {
            center: Vector3::new(250.0, 200.0, 0.0),
            radius: 30.0,
            material: Material::Glass(white, 1.0),
        }),
        Shape::Quad(Quad {
            q: Vector3::new(555.0, 0.0, 0.0),
            u: Vector3::new(0.0, 555.0, 0.0),
            v: Vector3::new(0.0, 0.0, 555.0),
            material: Material::Diffuse(green, 1.5),
        }),
        Shape::Quad(Quad {
            q: Vector3::new(0.0, 0.0, 0.0),
            u: Vector3::new(0.0, 555.0, 0.0),
            v: Vector3::new(0.0, 0.0, 555.0),
            material: Material::Diffuse(red, 1.5),
        }),
        Shape::Quad(Quad {
            q: Vector3::new(0.0, 0.0, 0.0),
            u: Vector3::new(555.0, 0.0, 0.0),
            v: Vector3::new(0.0, 0.0, 555.0),
            material: Material::Diffuse(white, 1.5),
        }),
        Shape::Quad(Quad {
            q: Vector3::new(555.0, 555.0, 555.0),
            u: Vector3::new(-555.0, 0.0, 0.0),
            v: Vector3::new(0.0, 0.0, -555.0),
            material: Material::Diffuse(white, 1.5),
        }),
        Shape::Quad(Quad {
            q: Vector3::new(0.0, 0.0, 555.0),
            u: Vector3::new(555.0, 0.0, 0.0),
            v: Vector3::new(0.0, 555.0, 0.0),
            material: Material::Diffuse(white, 1.5),
        }),
        Shape::Quad(Quad {
            q: Vector3::new(343.0, 554.0, 332.0),
            u: Vector3::new(-130.0, 0.0, 0.0),
            v: Vector3::new(0.0, 0.0, -165.0),
            material: Material::Light(Vector3::new(1.0, 1.0, 1.0), 25.5),
        }),
    ];

    let camera = Camera::new(
        Vector3::new(278.0, 278.0, 0.0),
        Vector3::new(278.0, 278.0, -200.0),
        90.0,
        ASPECT_RATIO,
        args.width as u64,
    );

    draw_scene(
        camera,
        shapes.as_slice(),
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
    shapes: &[Shape],
    frame_buffer: &mut FrameBuffer,
    samples: usize,
    bounces: usize,
) -> Result<(), Box<dyn Error>> {
    let width = frame_buffer.width();
    frame_buffer
        .pixel_data_mut()
        .par_chunks_mut(width * 4)
        .enumerate()
        .for_each(|(y, data)| {
            eprintln!("rendering line {y}");
            for x in 0..width {
                let color = iter::repeat_with(|| {
                    camera.cast(x as f64, y as f64, OsRng.gen_range(0.0..1.0))
                })
                .map(|ray| ray_color(shapes, ray, None, bounces))
                .take(samples)
                .reduce(|acc, e| acc + e)
                .unwrap()
                    / samples as f64;

                let i = x * 4;
                let (r, g, b, a) = color.to_rgba32();
                data[i..i + 4].copy_from_slice(&[r, g, b, a]);
            }
        });

    Ok(())
}

fn ray_color(shapes: &[Shape], ray: Ray, skip: Option<Shape>, bounces: usize) -> Vector3<f64> {
    for shape in shapes {
        if matches!(skip, Some(skip) if skip == *shape) {
            continue;
        }

        match shape.hit(ray) {
            Some(hit) => match hit.material {
                Material::Diffuse(color, factor) => {
                    let direction = hit.normal + random_unit_vec();
                    if bounces > 0 {
                        return factor
                            * ray_color(
                                shapes,
                                Ray {
                                    origin: hit.point,
                                    direction,
                                },
                                Some(*shape),
                                bounces - 1,
                            )
                            .component_mul(&color);
                    } else {
                        break;
                    }
                }
                Material::Metal(color, fuzz) => {
                    if bounces > 0 {
                        let reflected =
                            ray.direction - (2.0 * ray.direction.dot(&hit.normal) * hit.normal);
                        let fuzzed = reflected.normalize() + (fuzz * random_unit_vec());
                        return ray_color(
                            shapes,
                            Ray {
                                origin: hit.point,
                                direction: fuzzed,
                            },
                            Some(*shape),
                            bounces - 1,
                        )
                        .component_mul(&color);
                    } else {
                        break;
                    }
                }
                Material::Glass(color, refraction) => {
                    let uv = ray.direction.normalize();
                    let cos_theta = -uv.dot(&hit.normal).min(1.0);
                    let out_perp = refraction * (uv + cos_theta * hit.normal);
                    let out_parallel =
                        -(1.0 - out_perp.magnitude_squared().abs().sqrt()) * hit.normal;
                    let refracted = out_perp + out_parallel;

                    if bounces > 0 {
                        return ray_color(
                            shapes,
                            Ray {
                                origin: hit.point,
                                direction: refracted,
                            },
                            Some(*shape),
                            bounces - 1,
                        )
                        .component_mul(&color);
                    } else {
                        break;
                    }
                }
                Material::Light(color, intensity) => {
                    if bounces > 0 {
                        let direction = hit.normal + random_unit_vec();
                        return ray_color(
                            shapes,
                            Ray {
                                origin: hit.point,
                                direction,
                            },
                            Some(*shape),
                            bounces - 1,
                        ) + color * intensity;
                    } else {
                        break;
                    }
                }
            },
            None => continue,
        }
    }

    Vector3::new(0.0, 0.0, 0.0)
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

fn random_unit_vec() -> Vector3<f64> {
    loop {
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
    }
}

fn random_material() -> Material {
    let material = OsRng.gen_range(0..3);

    let color = Vector3::new(
        OsRng.gen_range(0.0..1.0),
        OsRng.gen_range(0.0..1.0),
        OsRng.gen_range(0.0..1.0),
    );

    match material {
        0 => Material::Diffuse(color, OsRng.gen_range(0.0..1.0)),
        1 => Material::Metal(color, OsRng.gen_range(0.0..1.0)),
        2 => Material::Glass(color, OsRng.gen_range(0.0..10.0)),
        _ => unreachable!(),
    }
}
