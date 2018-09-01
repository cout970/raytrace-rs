extern crate cgmath;
extern crate image;
extern crate rand;
extern crate rayon;
extern crate time;

use image::ImageBuffer;
use material::Materials;
use renderer::ImageChunk;
use renderer::Renderer;
use scene::Camera;
use scene::Scene;
use solids::Solid;
use std::fs::File;
use std::path::Path;
use time::get_time;
use util::Vector3f;

mod material;
mod scene;
mod util;
mod solids;
mod raytrace;
mod renderer;

fn main() {
    let width  =  3840;
    let height = 2160;
    let aspect = (width as f32) / (height as f32);

    println!("Opening image: environ.jpg");
    let mut img = image::open(Path::new("environ.jpg")).unwrap();
    let env = img.as_mut_rgb8().unwrap();


    let camera = Camera::new(&Vector3f::new(0.0, 0.0, 0.0), &Vector3f::new(0.0, 0.0, -1.0), aspect);

    let mut spheres: Vec<Solid> = Vec::new();

    for i in 0..25 {
        spheres.push(
            Solid::Sphere {
                pos: Vector3f::new((i % 5) as f32 - 2.0, (i / 5) as f32 - 2.0, -3.0),
                radius: 0.4,
                material: Materials::Metalic  { albedo: Vector3f::new(1.0, 1.0, 1.0), roughness: i as f32 / 24.0 },
            }
        );
    }

    let scene = Scene {
        solids: spheres,
//        vec![
//            Solid::Sphere { pos: Vector3f::new(0.55, 0.0, -1.0), radius: 0.5, material: Materials::Lambertian { albedo: Vector3f::new(0.2, 0.2, 0.9) } },
//            Solid::Sphere { pos: Vector3f::new(-0.55, 0.0, -1.0), radius: 0.5, material: Materials::Lambertian { albedo: Vector3f::new(0.9, 0.2, 0.2) } },
//            Solid::Plane { pos: Vector3f::new(0.0, -0.4, 0.0), normal: Vector3f::new(0.0, 1.0, 0.0), material: Materials::Metalic { albedo: Vector3f::new(0.8, 0.8, 0.8), roughness: 0.1 } },
//        ],
        environment: (env.width(), env.height(), env.iter().map(|r| *r).collect()),
    };

    println!("Starting render");
    let now = get_time();
    let buffer: ImageChunk = Renderer::render(&scene, &camera, (width, height));

    let after = get_time();
    let delta = (after - now);

    println!("Took: {:?}", delta);

    image::save_buffer("image.png", buffer.data.as_slice(), buffer.width, buffer.height, image::RGB(8)).unwrap();

    println!("Finished");
}
