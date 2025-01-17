use crate::vector; 

pub struct Ray {
    origin: vector::Vec3,
    direction: vector::Vec3,
}

impl Ray {
    pub fn new(origin: vector::Vec3, direction: vector::Vec3) -> Ray {
        Ray {
            origin,
            direction,
        }
    }

    pub fn at(&self, time: vector::fsize) -> vector::Vec3 {
        &self.origin + time * &self.direction
    }

    // Getters 
    pub fn origin(&self) -> &vector::Vec3 {
        &self.origin
    }

    pub fn direction(&self) -> &vector::Vec3 {
        &self.direction
    }
}