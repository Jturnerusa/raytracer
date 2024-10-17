use nalgebra::Vector3;

use crate::Ray;

const VIEWPORT_HEIGHT: f64 = 1.0;

#[derive(Clone, Debug)]
pub struct Camera {
    at: Vector3<f64>,
    from: Vector3<f64>,
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
        at: Vector3<f64>,
        from: Vector3<f64>,
        fov: f64,
        aspect_ratio: f64,
        image_width: u64,
    ) -> Self {
        let image_height: u64 = image_width / aspect_ratio as u64;

        let focal_length = (from - at).magnitude();

        let viewport_height = 2.0 * (fov.to_radians() / 2.0).tan() * focal_length;
        let viewport_width = viewport_height * image_width as f64 / image_height as f64;

        let w = (from - at).normalize();
        let u = Vector3::new(0.0, 1.0, 0.0).cross(&w).normalize();
        let v = w.cross(&u);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left = from - (focal_length * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel_0 = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            at,
            from,
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
        let direction = pixel_sampled - self.from;
        Ray {
            origin: pixel_sampled,
            direction,
        }
    }
}
