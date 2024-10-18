use crate::{
    hit::{Material, Record},
    Hit,
};
use nalgebra::Vector3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Quad {
    pub q: Vector3<f64>,
    pub u: Vector3<f64>,
    pub v: Vector3<f64>,
    pub material: Material,
}

impl Hit for Quad {
    fn hit(&self, ray: crate::ray::Ray) -> Option<crate::hit::Record> {
        let n = self.u.cross(&self.v);
        let normal = n.normalize();
        let d = normal.dot(&self.q);
        let w = n / n.dot(&n);

        let denom = normal.dot(&ray.direction);

        if denom.abs() < 1e-8 {
            return None;
        }

        let t = (d - normal.dot(&ray.origin)) / denom;

        let intersection = ray.at(t);
        let p = intersection - self.q;
        let alpha = w.dot(&p.cross(&self.v));
        let beta = w.dot(&self.u.cross(&p));

        if (0.0..=1.0).contains(&alpha) && (0.0..=1.0).contains(&beta) {
            Some(Record {
                point: intersection,
                normal,
                front: true,
                t,
                material: self.material,
            })
        } else {
            None
        }
    }
}
