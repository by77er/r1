pub mod renderer;

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use renderer::camera;
use renderer::types::*;
use vec::Vec3;

use std::f64::consts::PI;
use std::time::Duration;

fn main() {
  // SDL setup
  let sdl_context = sdl2::init().unwrap();
  let video_subsystem = sdl_context.video().unwrap();
  let window = video_subsystem
    .window("r1 renderer", 1000, 1000)
    .position_centered()
    .build()
    .unwrap();

  let mut canvas = window.into_canvas().build().unwrap();

  // Mesh and camera
  let mut cube = polygon::Mesh::new(
    Vec3::new(0.0, 0.0, 2.5), // center
    polygon::gen_cube(1.0) // Mesh parts
  );

  let cam = camera::Camera::new(
    Vec3::new(0.0, 0.0, 0.0),
    (50.0 / 180.0) * 2.0 * PI,
    Vec3::new(0.0, 0.0, PI),
  );

  // SDL loop
  canvas.set_draw_color(Color::RGB(0, 0, 0));
  canvas.clear();
  canvas.present();
  let mut event_pump = sdl_context.event_pump().unwrap();
  'running: loop {
    // Rendering 
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    // cam.recalculate_matrices();
    cube.rotation.x += 0.012;
    cube.rotation.y += 0.014;
    cube.rotation.z -= 0.016;
    cube.draw(&mut canvas, &cam);
    
    for event in event_pump.poll_iter() {
      match event {
        Event::Quit {..} |
        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
          break 'running
        },
        _ => {}
      }
    }
    canvas.present();
    std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
  }
}