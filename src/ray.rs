use nalgebra::Vector3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray {
    pub origin: Vector3<f64>,
    pub direction: Vector3<f64>,
}

impl Ray {
    pub(crate) fn at(&self, t: f64) -> Vector3<f64> {
        self.origin + t * self.direction
    }
}
