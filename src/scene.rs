use raytrace::Ray;
use solids::Solid;
use std::f32::consts::PI;
use std::sync::Arc;
use util::Vector3f;
use cgmath::InnerSpace;

pub struct Scene {
    pub solids: Vec<Solid>,
    pub environment: (u32, u32, Vec<u8>),
}

pub struct Camera {
    pub pos: Vector3f,
    pub look_at: Vector3f,
    pub corner: Vector3f,
    pub hor: Vector3f,
    pub ver: Vector3f,
}

impl Camera {
    pub fn new(pos: &Vector3f, look_at: &Vector3f, aspect: f32) -> Camera {
        Camera {
            pos: pos.clone(),
            look_at: look_at.clone(),
            corner: Vector3f::new(-1.0 * aspect, -1.0, -1.0),
            hor: Vector3f::new(2.0 * aspect, 0.0, 0.0),
            ver: Vector3f::new(0.0, 2.0, 0.0),
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(&self.pos, &(self.corner + self.hor * u + self.ver * v).normalize())
    }
}

impl Scene {
    pub fn get_color(&self, dir: &Vector3f) -> Vector3f {
        let (width, height, ref data) = self.environment;

        let u: f32 = 0.5 + dir.z.atan2(dir.x) / (2.0 * PI);
        let v: f32 = dir.y * 0.5 + 0.5;

        let iu = (u.max(0.0).min(0.99999) * width as f32).floor() as usize;
        let iv_pre = (v.max(0.0).min(0.99999) * height as f32).floor() as i32;
        let iv = (height as i32 - iv_pre - 1) as usize;

        Vector3f::new(
            data[(iu + iv * (width as usize)) * 3 + 0] as f32 / 255.0,
            data[(iu + iv * (width as usize)) * 3 + 1] as f32 / 255.0,
            data[(iu + iv * (width as usize)) * 3 + 2] as f32 / 255.0,
        )
    }
}