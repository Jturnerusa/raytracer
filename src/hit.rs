use std::ops::Range;

use crate::{frame::Rgba32, Ray};
use nalgebra::Vector3;

pub(crate) trait Hit {
    fn hit(&self, ray: Ray, interval: Range<f64>) -> Option<Record>;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Material {
    Diffuse(Vector3<f64>, f64),
    Metal(Vector3<f64>, f64),
    Glass(Vector3<f64>, f64),
    Light(Vector3<f64>, f64),
}

pub(crate) struct Record {
    pub(crate) point: Vector3<f64>,
    pub(crate) normal: Vector3<f64>,
    pub(crate) t: f64,
    pub(crate) front: bool,
    pub(crate) material: Material,
}
