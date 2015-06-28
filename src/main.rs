extern crate sdl2;
extern crate glium;
extern crate libc;
extern crate glium_sdl2;

use sdl2::video::{Window, WindowPos};
use sdl2::keyboard::Keycode;
use std::rc::Rc;
use glium::Surface;
use glium_sdl2::DisplayBuild;

fn load_file_to_string(file_path : &str) -> String {
  use std::error::Error;
  use std::fs::File;
  use std::io::prelude::*;
  use std::path::Path;
  
  let path = Path::new(file_path);
  let display = path.display();

  // Open the path in read-only mode, returns `io::Result<File>`
  let mut file = match File::open(&path) {
      // The `description` method of `io::Error` returns a string that
      // describes the error
      Err(why) => panic!("Couldn't open file: {}: {}", display, Error::description(&why)),
      Ok(file) => file,
  };

  // Read the file contents into a string, returns `io::Result<usize>`
  let mut s = String::new();
  match file.read_to_string(&mut s) {
      Err(why) => panic!("Couldn't read {}: {}", display, Error::description(&why)),
      Ok(_) => s,
  }
}

pub fn main() {
  let mut sdl_context = sdl2::init().video().unwrap();

  let mut window = sdl_context.window("game", 800, 600)
    .build_glium()
    .unwrap();
  loop {
    let mut target = window.draw();
    target.clear_color(0.0, 1.0, 0.0, 1.0);
    target.finish();

    for event in sdl_context.event_pump().poll_iter() {
      use sdl2::event::Event;
      match event {
        Event::Quit {..} | Event::KeyDown {..} => {
          return;
        },
        _ => {}
      }
    }
  }
}