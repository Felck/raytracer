use super::interval::Interval;
use super::materials::Material;
use super::vec3::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}

pub struct HitRecord<'a> {
    pub material: &'a Material,
    pub t: f64,
    pub point: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
}

impl<'a> HitRecord<'a> {
    /// out_normal needs to be a unit vector
    pub fn new(t: f64, ray: &Ray, point: Vec3, out_normal: Vec3, material: &'a Material) -> Self {
        let front_face = ray.direction.dot(&out_normal) < 0.0;
        Self {
            t,
            point,
            normal: if front_face { out_normal } else { -out_normal },
            front_face,
            material,
        }
    }
}

pub trait Hittable: Sync {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord>;
}

impl Hittable for Vec<Box<dyn Hittable>> {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut closest_so_far = ray_t.max;
        let mut hit_record = None;
        for obj in self {
            if let Some(hit) = obj.hit(ray, Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = hit.t;
                hit_record = Some(hit);
            }
        }
        hit_record
    }
}
