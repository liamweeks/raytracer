

use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Point3;



pub struct Sphere {
    pub centre: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(centre: Point3, radius: f64) -> Self {
        return Sphere {
            centre: centre,
            radius: radius
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = ray.origin - self.centre;
        let a = ray.direction.len_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.len_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        let disc_sqrt = discriminant.sqrt();
        let root = (-half_b + disc_sqrt) / a;

        if discriminant < 0.0 {
            return false;
        } else if root < t_min || t_max < root {
            return false;
        } else {
            rec.t = root;
            rec.point = ray.at(rec.t);
            let outward_normal = (rec.point - self.centre) / self.radius;
            rec.set_face_normal(*ray, outward_normal);
            rec.normal = (rec.point - self.centre) / self.radius;

            return true;
        }

    }
}