use crate::{color, vector};
use crate::ray;

pub struct HitRecord {
    point: vector::Vec3,
    normal: vector::Vec3,
    time: vector::fsize,
    hit_front_face: bool,
}

// This would be interesting to change to a trait, with new() definitions being created by each geometry 
impl HitRecord {
    fn set_face_normal(&mut self, ray: &ray::Ray) {
        self.hit_front_face = vector::Vec3::dot(ray.direction(), &self.normal) < 0.0 ;

        if !self.hit_front_face {
            self.normal = -&self.normal;
        } // else unchanged 
    }

    pub fn new(point: vector::Vec3, normal: vector::Vec3, time: vector::fsize, ray: &ray::Ray) -> HitRecord {
        let mut record = HitRecord {
            point,
            normal,
            time,
            hit_front_face: false,
        };

        record.set_face_normal(ray);

        record
    }

    pub fn normal(&self) -> &vector::Vec3 {
        &self.normal
    }

    pub fn normal_as_color(&self) -> color::Color3 {
        color::Color3(self.normal.x(), self.normal.y(), self.normal.z())
    }
}

pub trait Hittable {
    fn hit(&self, ray: &ray::Ray, time_min: vector::fsize, time_max: vector::fsize) -> Option<HitRecord>;
}


use std::rc::Rc;
pub struct HitList {
    models: Vec<Rc<dyn Hittable>>,
}

impl Hittable for HitList {
    fn hit(&self, ray: &ray::Ray, time_min: vector::fsize, time_max: vector::fsize) -> Option<HitRecord> {
        let mut upper_bound = time_max;
        let mut current_hit: Option<HitRecord> = None;

        for model in &self.models {
            if let Some(hit) = model.hit(ray, time_min, upper_bound) {
                upper_bound = hit.time; 
                current_hit = Some(hit);
            }
        }

        current_hit
    }
}

impl HitList {
    pub fn new() -> HitList {
        HitList {
            models: vec![]
        }
    }

    pub fn clear(&mut self) {
        self.models.clear();
    }

    pub fn add(&mut self, model: Rc<dyn Hittable>) {
        self.models.push(model);
    }
}