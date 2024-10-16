use crate::{
    hit::{Material, Record},
    Hit, Ray,
};
use nalgebra::Vector3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere {
    pub center: Vector3<f64>,
    pub radius: f64,
    pub material: Material,
}

impl Hit for Sphere {
    fn hit(&self, ray: Ray) -> Option<crate::hit::Record> {
        let oc = self.center - ray.origin;
        let a = ray.direction.dot(&ray.direction);
        let h = ray.direction.dot(&oc);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let d = (h * h) - (a * c);

        if d < 0.0 {
            None
        } else {
            let root = (h - d.sqrt()) / a;
            let point = ray.at(root);
            let normal = (point - self.center) / self.radius;
            Some(Record {
                point,
                normal,
                t: root,
                front: ray.direction.dot(&normal) > 0.0,
                material: self.material,
            })
        }
    }
}
