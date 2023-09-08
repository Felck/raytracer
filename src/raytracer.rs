extern crate sdl2;

use rayon::prelude::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;

pub mod camera;
pub mod color;
pub mod config;
pub mod interval;
pub mod materials;
pub mod ray;
pub mod sphere;
pub mod vec3;

use config::Config;

pub fn run(config: &Config) -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Raytracer", config.width as u32, config.height as u32)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator
        .create_texture_streaming(
            PixelFormatEnum::RGB24,
            config.width as u32,
            config.height as u32,
        )
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        texture.with_lock(None, |buffer: &mut [u8], _| {
            let bands: Vec<(usize, &mut [u8])> = buffer
                .chunks_mut(config.camera.img_width * 3)
                .enumerate()
                .collect();

            bands.into_par_iter().for_each_init(
                || rand::thread_rng(),
                |rng, (y, band)| config.camera.render(y, band, &config.world, rng),
            );
        })?;

        canvas.clear();
        canvas.copy(&texture, None, None)?;
        canvas.present();
    }

    Ok(())
}
