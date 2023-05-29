use crate::{vec3::{Point3, Vec3}, ray::Ray};


#[derive(Copy, Debug, PartialEq, Clone, Default)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}


impl HitRecord {
    

    pub fn set_face_normal(&mut self, ray: Ray, outward_normal: Vec3) {
        let front_face = ray.direction.dot(&outward_normal) < 0.0;

        
        self.normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

    }
}


pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {true}
}