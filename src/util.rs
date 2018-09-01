use cgmath::Vector3;
use rand::thread_rng;
use rand::Rng;

pub type Vector3f = Vector3<f32>;


pub fn randf() -> f32 {
    thread_rng().gen::<f32>()
}