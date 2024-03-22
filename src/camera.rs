use nalgebra::Vector3;

use crate::Ray;

const VIEWPORT_HEIGHT: f64 = 1.0;

#[derive(Clone, Debug)]
pub struct Camera {
    center: Vector3<f64>,
    image_width: u64,
    image_height: u64,
    viewport_width: f64,
    viewport_height: f64,
    viewport_u: Vector3<f64>,
    viewport_v: Vector3<f64>,
    pixel_delta_u: Vector3<f64>,
    pixel_delta_v: Vector3<f64>,
    pixel_0: Vector3<f64>,
}

impl Camera {
    pub fn new(
        center: Vector3<f64>,
        aspect_ratio: f64,
        focal_length: f64,
        image_width: u64,
    ) -> Self {
        let image_height: u64 = image_width / aspect_ratio as u64;
        let viewport_height = VIEWPORT_HEIGHT;
        let viewport_width = viewport_height * image_width as f64 / image_height as f64;
        let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;
        let viewport_upper_left =
            center - Vector3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel_0 = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
        Self {
            center,
            image_width,
            image_height,
            viewport_width,
            viewport_height,
            viewport_u,
            viewport_v,
            pixel_delta_u,
            pixel_delta_v,
            pixel_0,
        }
    }

    pub fn cast(&self, x: f64, y: f64, r: f64) -> Ray {
        let px = -0.5 + r;
        let py = -0.5 + r;
        let pixel_center = self.pixel_0 + (x * self.pixel_delta_u) + (y * self.pixel_delta_v);
        let pixel_sampled = pixel_center + (self.pixel_delta_u * px) + (self.pixel_delta_v * py);
        let direction = pixel_sampled - self.center;
        Ray {
            origin: pixel_sampled,
            direction,
        }
    }
}
