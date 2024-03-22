use crate::Ray;
use nalgebra::Vector3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere {
    pub center: Vector3<f64>,
    pub radius: f64,
}

impl Sphere {
    pub fn intersects(&self, ray: Ray) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let d = b * b - 4.0 * a * c;
        d >= 0.0
    }
}
