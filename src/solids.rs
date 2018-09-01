use cgmath::InnerSpace;
use raytrace::HitInfo;
use raytrace::Ray;
use std::iter::IntoIterator;
use std::vec::Vec;
use util::Vector3f;
use material::Materials;

pub enum Solid {
    Sphere {
        pos: Vector3f,
        radius: f32,
        material: Materials,
    },

    Plane {
        pos: Vector3f,
        normal: Vector3f,
        material: Materials,
    },
}

impl Solid {
    pub fn hit(&self, ray: &Ray) -> Option<Vec<HitInfo>> {
        match self {
            Solid::Sphere { pos, radius, material } => {
                let oc = ray.origin - pos;
                let a = ray.dir.dot(ray.dir);
                let b = 2.0 * oc.dot(ray.dir);
                let c = oc.dot(oc) - radius * radius;
                let disc = b * b - 4.0 * a * c;

                let t1 = (-b - disc.sqrt()) / (2.0 * a);
                let t2 = (-b + disc.sqrt()) / (2.0 * a);

                let mut hits: Vec<HitInfo> = Vec::with_capacity(2);

                if t1 > ray.t_min && t1 < ray.t_max {
                    let hit_pos = ray.point_at(t1);
                    //Calculate normal
                    let normal = ((hit_pos - pos) / (*radius)).normalize();
                    //return hitinfo
                    let info = HitInfo {
                        t: t1,
                        pos: hit_pos,
                        normal,
                        material: *material,
                    };

                    hits.push(info);
                }

                if t2 > ray.t_min && t2 < ray.t_max {
                    let hit_pos = ray.point_at(t2);
                    //Calculate normal
                    let normal = ((hit_pos - pos) / (*radius)).normalize();
                    //return hitinfo
                    let info = HitInfo {
                        t: t2,
                        pos: hit_pos,
                        normal,
                        material: *material,
                    };

                    hits.push(info);
                }

                if hits.is_empty() { None } else { Some(hits) }
            }
            Solid::Plane { pos, normal, material } => {
                let nb = normal.dot(ray.dir);
                if nb != 0.0 {
                    let na = normal.dot(ray.origin);
                    let nc = normal.dot(*pos);
                    let t = (nc - na) / nb;

                    if t > ray.t_min && t < ray.t_max {
                        let hit = ray.point_at(t);

                        let info = HitInfo {
                            t,
                            pos: hit,
                            normal: normal.clone(),
                            material: *material,
                        };


                        Some(vec![info])
                    } else { None }

                } else { None }
            }
            _ => None
        }
    }
}
