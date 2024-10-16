use crate::Ray;
use nalgebra::Vector3;

pub(crate) trait Hit {
    fn hit(&self, ray: Ray) -> Option<Record>;
}

pub(crate) struct Record {
    pub(crate) point: Vector3<f64>,
    pub(crate) normal: Vector3<f64>,
    pub(crate) t: f64,
    pub(crate) front: bool,
}
