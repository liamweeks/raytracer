use std::{sync::Arc, collections::btree_set::Difference};

use crate::{vec3::*, hittable::{HitRecord, Hittable}};


#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Point3 {
        return self.origin + t * self.direction;
    }


    pub fn new(origin: Point3, direction: Vec3) -> Self {
        return Self {
            origin,
            direction
        };
    }

    pub fn hit_sphere(&self, centre: Point3, radius: f64) -> f64 {
        /* 

        a sphere centered at the origin of radius 𝑅
        is (𝑥^2)+(𝑦^2)+(𝑧^2) = (𝑅^2)
        Put another way, if a given point (𝑥,𝑦,𝑧)
        is on the sphere, then 𝑥2+𝑦2+𝑧2=𝑅2
        If the given point (𝑥,𝑦,𝑧)
        is inside the sphere, then 𝑥2+𝑦2+𝑧2<𝑅2
        , and if a given point (𝑥,𝑦,𝑧)
        is outside the sphere, then 𝑥2+𝑦2+𝑧2>𝑅2

        (𝑡^2)𝐛⋅𝐛+2𝑡𝐛⋅(𝐀−𝐂)+(𝐀−𝐂)⋅(𝐀−𝐂)−(r^2)=0
        
        Vectors A and C are known, so we solve for t.
        the roots represent the places where the ray
        intersects with the surface of the sphere.

        */
        

        // This was originally a quadratic formula but I factored out the 2.0 to make everything a little faster


        let oc = self.origin - centre;
        let a = self.direction.len_squared();

        let half_b = oc.dot(&self.direction);
        let c = oc.len_squared() - radius * radius;
        let discriminant =  half_b * half_b - a * c;

        if discriminant < 0.0 {
            return -1.0;
        } else {
            return (-half_b - discriminant.sqrt()) / a;
        }

    }
}

