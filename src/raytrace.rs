use cgmath::InnerSpace;
use material::ScatterResult;
use scene::Scene;
use solids::Solid;
use std::cmp::Ord;
use std::cmp::Ordering::Equal;
use std::f32::MAX;
use util::randf;
use util::Vector3f;
use material::Materials;

pub struct Ray {
    pub origin: Vector3f,
    pub dir: Vector3f,
    pub t_min: f32,
    pub t_max: f32,
}

pub struct HitInfo {
    pub t: f32,
    pub pos: Vector3f,
    pub normal: Vector3f,
    pub material: Materials,
}


impl Ray {
    pub fn new(origin: &Vector3f, dir: &Vector3f) -> Ray {
        Ray {
            origin: origin.clone(),
            dir: dir.normalize(),
            t_min: 0.001,
            t_max: MAX,
        }
    }

    pub fn point_at(&self, t: f32) -> Vector3f {
        self.origin + self.dir * t
    }
}

pub struct RayTracer;


impl RayTracer {
    pub fn shoot(scene: &Scene, ray: &Ray, bounces: i32) -> Vector3f {
        let hits: Vec<HitInfo> = scene.solids.iter()
            .map(|s| s.hit(ray))
            .filter(|o| o.is_some())
            .flat_map(|i| i.unwrap())
            .collect();

        if !hits.is_empty() {
            if bounces <= 0 {
                return Vector3f::new(0.0, 0.0, 0.0);
            }

            let hit_option = hits.iter().min_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(Equal));

            if let Some(hit) = hit_option {
                let result = hit.material.scatter(ray, hit);

                match result {
                    Some(ScatterResult { scatter, attenuation }) => {
                        let color = RayTracer::shoot(scene, &Ray::new(&hit.pos, &scatter), bounces - 1);
                        return Vector3f::new(
                            color.x * attenuation.x,
                            color.y * attenuation.y,
                            color.z * attenuation.z,
                        );
                    }
                    None => return Vector3f::new(0.0, 0.0, 0.0)
                };
            }
        }

        scene.get_color(&ray.dir)
//
//        let t = ray.dir.y * 0.5 + 1.0;
//        Vector3f::new(1.0, 1.0, 1.0) * (1.0 - t) + Vector3f::new(0.5, 0.7, 1.0) * t
    }
}

pub fn rand_in_unit_sphere() -> Vector3f {
    loop {
        let p = Vector3f::new(
            randf() * 2.0 - 1.0,
            randf() * 2.0 - 1.0,
            randf() * 2.0 - 1.0,
        );

        if p.dot(p) < 1.0 {
            return p;
        }
    }
}