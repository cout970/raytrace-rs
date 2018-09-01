use cgmath::InnerSpace;
use raytrace::HitInfo;
use raytrace::rand_in_unit_sphere;
use raytrace::Ray;
use util::Vector3f;

pub struct ScatterResult {
    pub scatter: Vector3f,
    pub attenuation: Vector3f,
}

#[derive(Clone, Copy)]
pub enum Materials {
    Metalic { albedo: Vector3f, roughness: f32 },
    Lambertian { albedo: Vector3f },
}

impl Materials {
    pub fn scatter(&self, ray: &Ray, hit: &HitInfo) -> Option<ScatterResult> {
        match self {
            Materials::Metalic { albedo, roughness } => {
                let reflected = reflect(ray.dir, hit.normal).normalize() + rand_roughness_vec(*roughness);

                if reflected.dot(hit.normal) > 0.0 {
                    Some(ScatterResult { scatter: reflected, attenuation: albedo.clone() })
                } else {
                    None
                }
            }
            Materials::Lambertian { albedo } => {
                let scatter = (hit.pos + hit.normal + rand_in_unit_sphere()).normalize();
                Some(ScatterResult { scatter, attenuation: albedo.clone() })
            }
        }
    }
}

fn reflect(a: Vector3f, b: Vector3f) -> Vector3f {
    let dot = a.dot(b);
    Vector3f::new(
        a.x - (dot + dot) * b.x,
        a.y - (dot + dot) * b.y,
        a.z - (dot + dot) * b.z,
    )
}

fn rand_roughness_vec(roughness: f32) -> Vector3f {
    if roughness > 0.0 {
        rand_in_unit_sphere() * roughness
    } else {
        Vector3f::new(0.0, 0.0, 0.0)
    }
}