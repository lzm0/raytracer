use crate::ray::*;
use crate::vec3::*;

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            p: Point3::zero(),
            normal: Vec3::zero(),
            t: 0.0,
            front_face: false,
        }
    }
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

#[allow(unused)]
pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, mut rec: &mut HitRecord) -> bool {
        false
    }
}
