use image::ImageBuffer;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;
use raytrace::RayTracer;
use scene::Camera;
use scene::Scene;
use std::sync::Arc;
use util::randf;
use util::Vector3f;

pub struct Renderer;

#[derive(Clone)]
pub struct ImageChunk {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}


const CHUNK_SIZE: u32 = 32;
const RAYS_PER_PIXEL: u32 = 2000;

impl Renderer {
    pub fn render(scene: &Scene, camera: &Camera, size: (u32, u32)) -> ImageChunk {
        let mut coords: Vec<(u32, u32, u32, u32)> = Vec::new();
        let (g_width, g_height) = size;

        let chunks_x = (size.0 as f32 / CHUNK_SIZE as f32).ceil() as u32;
        let chunks_y = (size.1 as f32 / CHUNK_SIZE as f32).ceil() as u32;

        for x in 0..chunks_x {
            for y in 0..chunks_y {
                let g_x = x * CHUNK_SIZE;
                let g_y = y * CHUNK_SIZE;

                let width = if g_x + CHUNK_SIZE > g_width { g_width % CHUNK_SIZE } else { CHUNK_SIZE };
                let height = if g_y + CHUNK_SIZE > g_height { g_height % CHUNK_SIZE } else { CHUNK_SIZE };

                if width != 0 && height != 0 {
                    coords.push((g_x, g_y, width, height));
                }
            }
        }

        println!("{} chunks", coords.len());

        let chunks: Vec<ImageChunk> = coords.par_iter()
            .map(|c| Renderer::render_chunk(scene, camera, c, &size))
            .collect();

        let mut global = ImageChunk {
            x: 0,
            y: 0,
            width: g_width,
            height: g_height,
            data: vec![255u8; (g_width * g_height * 3) as usize],
        };

        for chunk in chunks.iter() {
            Renderer::sub_image(&mut global, chunk);
        }
        global
    }

    fn sub_image(global: &mut ImageChunk, chunk: &ImageChunk) {
        for i in 0..chunk.width {
            for j in 0..chunk.height {
                let x = i + chunk.x;
                let y = global.height - (j + chunk.y) - 1;
                let g_base = (x + y * global.width) as usize;
                let c_base = (i + j * chunk.width) as usize;

                if g_base * 3 >= global.data.len() {
                    panic!();
                }

                global.data[g_base * 3 + 0] = chunk.data[c_base * 3 + 0];
                global.data[g_base * 3 + 1] = chunk.data[c_base * 3 + 1];
                global.data[g_base * 3 + 2] = chunk.data[c_base * 3 + 2];
            }
        }
    }

    fn render_chunk(scene: &Scene, camera: &Camera, chunk: &(u32, u32, u32, u32), canvas: &(u32, u32)) -> ImageChunk {
        let (x, y, width, height) = *chunk;
        println!("Rendering chunk: ({}, {})", x / CHUNK_SIZE, y / CHUNK_SIZE);
        let mut data = vec![0u8; (width * height * 3) as usize];

        for i in 0..width {
            for j in 0..height {
                let mut color = Vector3f::new(0.0, 0.0, 0.0);

                for n in 0..RAYS_PER_PIXEL {
                    let u = ((i + x) as f32 + randf()) / canvas.0 as f32;
                    let v = ((j + y) as f32 + randf()) / canvas.1 as f32;

                    color += RayTracer::shoot(scene, &camera.get_ray(u, v), 50);
                }

                color /= RAYS_PER_PIXEL as f32;
                let base = (i + j * width) as usize;
                data[base * 3 + 0] = (color.x.sqrt() * 255.0) as u8;
                data[base * 3 + 1] = (color.y.sqrt() * 255.0) as u8;
                data[base * 3 + 2] = (color.z.sqrt() * 255.0) as u8;
            }
        }

        ImageChunk { x, y, width, height, data }
    }
}
