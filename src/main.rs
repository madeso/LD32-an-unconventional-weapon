extern crate sdl2;
extern crate glium;
extern crate libc;
extern crate glium_sdl2;

extern crate protobuf;
mod settings;

use glium::Surface;
use glium_sdl2::DisplayBuild;

use std::fs::File;
use std::io;
use std::path::Path;

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
      Ok(file) => file
  };

  // Read the file contents into a string, returns `io::Result<usize>`
  let mut s = String::new();
  match file.read_to_string(&mut s) {
      Err(why) => panic!("Couldn't read {}: {}", display, Error::description(&why)),
      Ok(_) => s,
  }
}

fn load_settings(path:&str) -> settings::Settings {
  match File::open(&Path::new(path)) {
    Ok(mut is) => {
      match protobuf::parse_from_reader::<settings::Settings>(&mut is) {
        Err(err) => {
          println!("Unable to parse binary: {:?}", err);
          settings::Settings::new()
        },
        Ok(sett) => sett,
      }
    },
    Err(err) => {
      println!("Unable to load settings file: {}", err);
      settings::Settings::new()
    },
  }
}

pub fn main() {
  let mut sdl_context = sdl2::init().video().unwrap();
  
  // let settings = settings::Settings::new();
  // let mut d = "".to_string();
  // protobuf::text_format::print_to(&settings, &mut d);
  let settings = load_settings("settings.bin");
  
  let window = sdl_context.window("game", settings.get_width(), settings.get_height())
    .build_glium()
    .unwrap();
  
  loop {
    let mut target = window.draw();
    target.clear_color(0.0, 1.0, 0.0, 1.0); // 1
    target.clear_color(0.0, 1.0, 0.0, 1.0); // 2
    target.clear_color(0.0, 1.0, 0.0, 1.0); // 3
    target.clear_color(0.0, 1.0, 0.0, 1.0); // 4
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
