use crate::vector;
use crate::ray;
use crate::model::{
    HitRecord,
    Hittable,
};

pub struct Sphere {
    center: vector::Vec3,
    radius: vector::fsize,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &ray::Ray, time_min: vector::fsize, time_max: vector::fsize) -> Option<HitRecord> {
        let offset = ray.origin() - &self.center;

        let a = ray.direction().length_squared();
        let half_b = vector::Vec3::dot(&offset, ray.direction());
        let c = offset.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant >= 0.0 {
            let mut root = ( -half_b - discriminant.sqrt() ) / a;
            
            if root < time_min || root > time_max {
                root = ( -half_b - discriminant.sqrt() ) / a;
                if root < time_min || root > time_max {
                    return None;
                }
            }

            let point = ray.at(root);
            let normal = (&point - &self.center) / self.radius;
            return Some(HitRecord::new(
                point,
                normal,
                root,
                ray
            ))
        }

        None
    }
}

impl Sphere {
    pub fn new(position: vector::Vec3, radius: vector::fsize) -> Sphere {
        Sphere {
            center: position,
            radius,
        }
    }
}